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
    let mut cap;

    if re_fail.is_match(text) {
        cap = re_fail.captures(text).unwrap(); 

    } else {
        cap = re_success.captures(text).unwrap();
    }
    println!("{:?}", cap);

    match cap.get(3).map_or("", |m| m.as_str()) {
        "OK" => {
            let total = cap.get(1).map_or("", |m| m.as_str()).parse().unwrap();
            println!("total: {}, error: 0, success: {}", total, total);
            ResponseMetrics { total: total, error: 0, success: total}
        },
        _ => { 
            let total = cap.get(1).map_or("", |m| m.as_str()).parse().unwrap();
            let error = cap.get(3).map_or("", |m| m.as_str()).parse().unwrap();
            println!("total: {}, error: {}, success: 0", total, error);
            ResponseMetrics { total: total, error: error, success: 0 }
        }, 
    }
}


