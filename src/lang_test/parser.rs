extern crate regex;
use self::regex::Regex;
use self::regex::Captures;

#[derive(RustcEncodable, RustcDecodable)]
pub struct ResponseMetrics {
    pub total: i8,
    pub error: i8,
    pub success: i8 
}

pub fn parse_py_rsp(text: &str) -> ResponseMetrics {

    let re_fail = Regex::new(r"Ran (\d) tests(.+|\n)=(\d)").unwrap(); 
    let re_success = Regex::new(r"Ran (\d) tests(.+|\n)(OK)").unwrap(); 
    let mut cap: Captures;

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
                    ResponseMetrics { total: 0, error: 1, success: 0 }

                }
            }
        } 
    }
}


