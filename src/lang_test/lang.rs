use std::{thread, time};
use std::fs;
use std::process::Command;
use std::path::Path;
use std::env;
use std::string::String;


pub struct Python {
    user: String,
    subject: String,
}

pub struct Scala {
    user: String,
    subject: String,
}

pub trait Lang {
    fn new(user: String, subject: String) -> Self;
    fn run_test(&self) -> String;
}

impl Lang for Python {
    fn new(user: String, subject: String) -> Python {
        Python { user: user, subject: subject}
    } 

    fn run_test(&self) -> String {
        let from = format!("/home/{}/Code-Fight/python/{}", self.user, self.subject);
        let to = format!("/test/{}/Code-Fight/python/{}", self.user, self.subject);
        println!("from => {}, to => {}", &from, &to);
        fs::copy(&from, &to);

        let result = Command::new("nosetests")
               .arg(&to)
               .output()
               .expect("cat error to start");
    
        let s = match String::from_utf8(result.stdout) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        s
        //println!("result: {:?}", &result);
        //let sleep_time = time::Duration::new(10, 0);;
        //thread::sleep(sleep_time);
    }
}

impl Lang for Scala {
    fn new(user: String, subject: String) -> Scala {
        Scala { user: user, subject: subject}
    } 

    fn run_test(&self) -> String {
        let from = format!("/home/{}/Code-Fight/scala/{}", self.user, self.subject);
        let to = format!("/test/{}/Code-Fight/scala/{}", self.user, self.subject);
        println!("from => {}, to => {}", &from, &to);

        fs::copy(&from, &to);
        
        let path = Path::new(&to);
        env::set_current_dir(&path).is_ok();

        let result = Command::new("sbt")
               .arg("test")
               .output()
               .expect("cat error to start");
    
        let s = match String::from_utf8(result.stdout) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        s
        //println!("result: {:?}", &result);
    }
}
