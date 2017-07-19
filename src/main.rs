extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;
use std::io::Read;

#[derive(RustcEncodable, RustcDecodable)]
struct RequestPayload {
    user: String,
    language: String,
    subject: String,
    solution: String
}

#[derive(RustcEncodable, RustcDecodable)]
struct ResponsePayload {
    responseCode: i8,
    responseMessage: String
}


mod lang_test;

fn main() {

    let mut router = Router::new();

    router.get("/", ping, "ping");
    router.post("/submit", submit, "submit");

    fn ping(_: &mut Request) -> IronResult<Response> {
        let message = ResponsePayload { responseCode: 0, responseMessage: "pong".to_string() }; 
        let payload = json::encode(&message).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    fn submit(req: &mut Request) -> IronResult<Response> {
        let mut body = String::new();
        req.body.read_to_string(&mut body).unwrap();

        let body: RequestPayload = json::decode(&body).unwrap();

        let responseMsg = lang_test::run_spark_test(&body.language, &body.user, &body.subject, &body.solution); 

        let message = ResponsePayload { responseCode: 0, responseMessage: responseMsg };
        let payload = json::encode(&message).unwrap();
        

        Ok(Response::with((status::Ok, payload)))
    }

    let _server = Iron::new(router).http("0.0.0.0:3000").unwrap();
    println!("On 3000");
}
