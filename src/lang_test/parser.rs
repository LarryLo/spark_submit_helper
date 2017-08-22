extern crate regex;
use self::regex::Regex;

#[derive(RustcEncodable, RustcDecodable)]
pub struct ResponseMetrics {
    pub total: i8,
    pub error: i8,
    pub success: i8 
}

pub fn parse_py_rsp(text: &str) -> ResponseMetrics {

    let re_fail = Regex::new(r"Ran (\d) tests(.+|\n)=(\d)").unwrap(); 
    let re_success = Regex::new(r"Ran (\d) tests(.+|\n)(OK)").unwrap(); 

    match re_fail.captures(text) {
        Some(cap) => {
            let total = cap.get(1).unwrap().as_str().parse().unwrap();
            let error = cap.get(3).unwrap().as_str().parse().unwrap();
            println!("total: {}, error: {}, success: 0", total, error);
            ResponseMetrics { total: total, error: error, success: 0 }
        },
        None => {
            match re_success.captures(text) {
                Some(cap) => {
                    let total = cap.get(1).unwrap().as_str().parse().unwrap(); 
                    println!("total: {}, error: 0, success: {}", total, total);
                    ResponseMetrics { total: total, error: 0, success: total}
                },
                None => {
                    ResponseMetrics { total: 127, error: 127, success: 0 }
                }
            }
        } 
    }
}

pub fn parse_scala_rsp(text: &str) -> ResponseMetrics {

    let re_compile_success = Regex::new(r"Total (\d), Failed (\d), Errors (\d), Passed (\d)").unwrap();

    match re_compile_success.captures(text) {
        Some(cap) => {
            let total: i8 = cap.get(1).unwrap().as_str().parse().unwrap();
            let fail: i8 = cap.get(2).unwrap().as_str().parse().unwrap();
            let error: i8 = cap.get(3).unwrap().as_str().parse().unwrap();
            let success: i8 = cap.get(4).unwrap().as_str().parse().unwrap();
            println!("total: {}, error: {}, success: {}", total, fail + error, success);
            ResponseMetrics { total: total, error: fail + error, success: success }
        },
        None => {
            ResponseMetrics { total: 127, error: 127, success: 0 }
        }
    } 
}

