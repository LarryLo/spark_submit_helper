extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;
use std::io::Read;

mod lang_test;

#[derive(RustcEncodable, RustcDecodable)]
struct RequestPayload {
    user: String,
    language: String,
    subject: String,
    solution: String
}

#[derive(RustcEncodable, RustcDecodable)]
struct ResponsePayload {
    response_code: i8,
    response_message: String,
    metrics: Option<lang_test::parser::ResponseMetrics>
}

fn main() {

    let mut router = Router::new();

    router.get("/", ping, "ping");
    router.post("/submit", submit, "submit");

    fn ping(_: &mut Request) -> IronResult<Response> {
        let message = ResponsePayload { response_code: 0, response_message: "pong".to_string(), metrics: None }; 
        let payload = json::encode(&message).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    fn submit(req: &mut Request) -> IronResult<Response> {
        let mut body = String::new();
        req.body.read_to_string(&mut body).unwrap();

        let body: RequestPayload = json::decode(&body).unwrap();

        let response_msg = lang_test::run_spark_test(&body.language, &body.user, &body.subject, &body.solution); 

        let message = match response_msg.error {
            // success
            0 => ResponsePayload { response_code: 0, response_message: "pass".to_string(), metrics: Some(response_msg) },
            // syntax error
            127 => ResponsePayload { response_code: 1, response_message: "syntax error".to_string(), metrics: Some(response_msg) },
            // test case failed
            _ => ResponsePayload { response_code: 2, response_message: "fail".to_string(), metrics: Some(response_msg) }
        };
        let payload = json::encode(&message).unwrap();

        Ok(Response::with((status::Ok, payload)))
    }

    let _server = Iron::new(router).http("0.0.0.0:3000").unwrap();
    println!("On 3000");
}
