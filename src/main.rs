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

//extern crate getopts;
//use getopts::Options;
use std::env;
use std::{thread, time};
use std::fs;
use std::process::Command;

mod lang_test;
//
//fn print_usage(program: &str, opts: Options) {
//    let brief = format!("Usage: {}  ", program);
//    print!("{}", opts.usage(&brief));
//}

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

        let responseMsg = lang_test::run_spark_test(&body.language, &body.user, &body.subject); 

        let message = ResponsePayload { responseCode: 0, responseMessage: responseMsg };
        let payload = json::encode(&message).unwrap();
        

        Ok(Response::with((status::Ok, payload)))
    }

    let _server = Iron::new(router).http("0.0.0.0:3000").unwrap();
    println!("On 3000");
    //let args: Vec<String> = env::args().collect();
    //let program = args[0].clone();
    //let competition_name = "Code-Fight";
    //let test_prefix_loc = "/test";

    //let mut opts = Options::new();
    //opts.optflag("h", "help", "print this help menu");
    //opts.reqopt("l", "language", "(required) select your language.", "[ python | scala ]");
    //opts.reqopt("s", "subject", "(required) set subject name to submit", "e.g. word_count");
    //let matches = match opts.parse(&args[1..]) {
    //    Ok(m) => { m }
    //    Err(f) => { print_usage(&program, opts); panic!(f.to_string()) }
    //};

    //if matches.opt_present("h") {
    //    print_usage(&program, opts);
    //    return;
    //}
    //
    //match env::var("USER") {
    //    Ok(user) => {
    //        
    //        match matches.opt_str("s") {
    //            Some(subject) => {

    //                match matches.opt_str("l") {
    //                    Some(language) => {

    //                       lang_test::run_spark_test(&language, &user, &subject); 

    //                    },
    //                    None => {
    //                        println!("No such language");
    //                    }
    //                };
    //            },
    //            None => {
    //                println!("No such subject");
    //            }
    //        }
    //    },
    //    Err(e) => println!("couldn't interpret {}: {}", "USER", e),
    //};

}
