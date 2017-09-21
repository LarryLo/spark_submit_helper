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
    fn run_test(&self, solution: &str, version: i8) -> String;
}

impl Lang for Python {
    fn new(user: String, subject: String) -> Python {
        Python { user: user, subject: subject}
    } 

    fn run_test(&self, solution: &str, version: i8) -> String {
        let to = format!("/test/{}/Code-Fight/python/{}", self.user, self.subject);
        println!("language => python{}, to => {}", version, &to);

        // write solution to solution.py
        echo(solution, &Path::new(&format!("{}/solution.py", &to)));

        let cmd = 
            match version {
                2 => "nosetests-2.7",
                _ => "nosetests"
            };
        // run test
        let result = 
            Command::new(&cmd)
               .arg(&to)
               .output()
               .ok()
               .expect("cat error to start");
    
        println!("user: {:?}, subject: {:?}, result: {:?}", self.user, self.subject, &result);
        format!("{:?}", &result)
    }
}

impl Lang for Scala {
    fn new(user: String, subject: String) -> Scala {
        Scala { user: user, subject: subject}
    } 

    fn run_test(&self, solution: &str, version: i8) -> String {
        let to = format!("/test/{}/Code-Fight/scala/{}", self.user, self.subject);
        println!("language => scala, to => {}", &to);
        
        // write solution to Solution.scala
        echo(solution, &Path::new(&format!("{}/src/main/scala/org/sparktw/codefight/Solution.scala", &to)));
        let path = Path::new(&to);

        // change location to subject
        // run test
        let result = 
            Command::new("sbt")
               .current_dir(&path)
               .arg("test")
               .output()
               .ok()
               .expect("cat error to start");
    
        println!("result: {:?}", &result);
        format!("{:?}", &result)
    }
}
