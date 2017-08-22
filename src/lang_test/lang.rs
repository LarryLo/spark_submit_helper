use std::fs;
use std::io;
use std::io::Write;
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

fn echo(solution: &str, path: &Path) -> io::Result<()> {
    let mut file = try!(fs::File::create(path));
    file.write_all(solution.as_bytes())
}

pub trait Lang {
    fn new(user: String, subject: String) -> Self;
    fn run_test(&self, solution: &str) -> String;
}

impl Lang for Python {
    fn new(user: String, subject: String) -> Python {
        Python { user: user, subject: subject}
    } 

    fn run_test(&self, solution: &str) -> String {
        let from = format!("/home/{}/Code-Fight/python/{}", self.user, self.subject);
        let to = format!("/test/{}/Code-Fight/python/{}", self.user, self.subject);
        println!("from => {}, to => {}", &from, &to);

        // write solution to solution.py
        echo(solution, &Path::new(&format!("{}/solution.py", &to)));

        // run test
        let result = Command::new("nosetests")
               .arg(&to)
               .output()
               .expect("cat error to start");
    
    //    println!("result: {:?}", &result);
        format!("{:?}", &result)
    }
}

impl Lang for Scala {
    fn new(user: String, subject: String) -> Scala {
        Scala { user: user, subject: subject}
    } 

    fn run_test(&self, solution: &str) -> String {
        let from = format!("/home/{}/Code-Fight/scala/{}", self.user, self.subject);
        let to = format!("/test/{}/Code-Fight/scala/{}", self.user, self.subject);
        println!("from => {}, to => {}", &from, &to);
        
        // write solution to Solution.scala
        echo(solution, &Path::new(&format!("{}/src/main/scala/org/sparktw/codefight/Solution.scala", &to)));

        // change location to subject
        let path = Path::new(&to);
        env::set_current_dir(&path).is_ok();

        // run test
        let result = Command::new("sbt")
               .arg("test")
               .output()
               .expect("cat error to start");
    
        println!("result: {:?}", &result);
        format!("{:?}", &result)
    }
}
