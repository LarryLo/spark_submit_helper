extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;
use std::io::Read;

mod lang_test;
mod user;

#[derive(RustcEncodable, RustcDecodable)]
struct SubjectTestRequestPayload {
    user: String,
    language: String,
    subject: String,
    solution: String
}

#[derive(RustcEncodable, RustcDecodable)]
struct SubjectTestResponsePayload {
    response_code: i8,
    response_message: String,
    metrics: Option<lang_test::parser::ResponseMetrics>
}

#[derive(RustcEncodable, RustcDecodable)]
struct UserRequestPayload {
    user: String
}

#[derive(RustcEncodable, RustcDecodable)]
struct UserResponsePayload {
    response_code: i8,
    response_message: String
}

fn main() {

    let mut router = Router::new();

    router.get("/", ping, "ping");
    router.post("/submit", submit, "submit");
    router.post("/create/user", create_user_subjects, "create user's subject folders");

    fn ping(_: &mut Request) -> IronResult<Response> {
        let message = SubjectTestResponsePayload { response_code: 0, response_message: "pong".to_string(), metrics: None }; 
        let payload = json::encode(&message).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    fn submit(req: &mut Request) -> IronResult<Response> {
        let mut body = String::new();
        req.body.read_to_string(&mut body).unwrap();

        let body: SubjectTestRequestPayload = json::decode(&body).unwrap();

        let response_msg = lang_test::run_spark_test(&body.language, &body.user, &body.subject, &body.solution); 

        let message = match response_msg.error {
            // success
            0 => SubjectTestResponsePayload { response_code: 0, response_message: "pass".to_string(), metrics: Some(response_msg) },
            // syntax error
            127 => SubjectTestResponsePayload { response_code: 1, response_message: "syntax error".to_string(), metrics: Some(response_msg) },
            // test case failed
            _ => SubjectTestResponsePayload { response_code: 2, response_message: "fail".to_string(), metrics: Some(response_msg) }
        };
        let payload = json::encode(&message).unwrap();

        Ok(Response::with((status::Ok, payload)))
    }

    fn create_user_subjects(req: &mut Request) -> IronResult<Response> {
        let mut body = String::new();
        req.body.read_to_string(&mut body).unwrap();

        let body: UserRequestPayload = json::decode(&body).unwrap();
        let result = user::create_subjects(&body.user);
        let message = match result.stderr.len() {
            // success
            0 => UserResponsePayload {response_code: 0, response_message: format!("Create {} subjects directories successfully.", &body.user)}, 
            _ => UserResponsePayload {response_code: 2, response_message: format!("{:?}", result.stderr)}       
        };
        let payload = json::encode(&message).unwrap();

        Ok(Response::with((status::Ok, payload)))
    } 

    let _server = Iron::new(router).http("0.0.0.0:3000").unwrap();
    println!("On 3000");
}
