use std::process::Command;
use std::process::Output;

pub fn create_subjects(user: &str) -> Output {
    let copy_from = "/test/source/Code-Fight";
    let copy_to = format!("/test/{}", user);

    let mkdir = 
        Command::new("mkdir")
            .arg("-p")
            .arg(&copy_to)
            .output()
            .ok()
            .expect("cat error to start");

    let result = 
        Command::new("cp")
            .arg("-r")
            .arg("-f")
            .arg(&copy_from)
            .arg(&copy_to)
            .output()
            .ok()
            .expect("cat error to start");

    return result;
}
