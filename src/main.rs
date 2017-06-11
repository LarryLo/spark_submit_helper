extern crate getopts;
use getopts::Options;
use std::env;
use std::{thread, time};
use std::fs;
use std::process::Command;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} -t [python|scala] -f FILE ", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let competition_name = "Code-Fight";
    let test_prefix_loc = "/test";

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.reqopt("l", "language", "select your language.", "[ python | scala ]");
    opts.reqopt("s", "subject", "set subject name to submit", "e.g. word_count");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { print_usage(&program, opts); panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    
    match env::var("USER") {
        Ok(user) => {
            
            match matches.opt_str("s") {
                Some(subject) => {

                    match matches.opt_str("l") {
                        Some(language) => {
                            let from = format!("/home/{}/{}/{}/{}", &user, &competition_name, language, subject);
                            let to = format!("{}/{}/{}/{}/{}", &test_prefix_loc, &user, &competition_name, language, subject);
                            println!("from => {}, to => {}", &from, &to);
                                
                            fs::copy(&from, &to);

                            Command::new("nosetests")
                                    .arg(&to)
                                    .spawn()
                                    .expect("cat error to start");
                        
                            let sleep_time = time::Duration::new(10, 0);;
                            thread::sleep(sleep_time);
                        },
                        None => {
                            println!("No such language");
                        }
                    };
                },
                None => {
                    println!("No such subject");
                }
            }
        },
        Err(e) => println!("couldn't interpret {}: {}", "USER", e),
    };

}
