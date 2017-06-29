use std::{thread, time};
use std::fs;
use std::process::Command;

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
    fn run_test(&self);
}

impl Lang for Python {
    fn new(user: String, subject: String) -> Python {
        Python { user: user, subject: subject}
    } 

    fn run_test(&self) {
        let from = format!("/home/{}/Code-Fight/python/{}", self.user, self.subject);
        let to = format!("/test/{}/Code-Fight/python/{}", self.user, self.subject);
        println!("from => {}, to => {}", &from, &to);
        fs::copy(&from, &to);

        let result = Command::new("nosetests")
               .arg(&to)
               .output()
               .expect("cat error to start");
    
        println!(&result);
        //let sleep_time = time::Duration::new(10, 0);;
        //thread::sleep(sleep_time);
    }
}

impl Lang for Scala {
    fn new(user: String, subject: String) -> Scala {
        Scala { user: user, subject: subject}
    } 

    fn run_test(&self) {
        let from = format!("/home/{}/Code-Fight/scala/{}", self.user, self.subject);
        let to = format!("/test/{}/Code-Fight/scala/{}", self.user, self.subject);
        println!("from => {}, to => {}", &from, &to);

        fs::copy(&from, &to);
        //Command::new("nosetests")
        //       .arg(&to)
        //       .spawn()
        //       .expect("cat error to start");
    
        let sleep_time = time::Duration::new(10, 0);;
        thread::sleep(sleep_time);
    
    }
}
