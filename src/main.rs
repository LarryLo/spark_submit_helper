extern crate getopts;
use getopts::Options;
use std::env;
use std::{thread, time};
use std::fs;
use std::process::Command;

mod lang_test;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {}  ", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let competition_name = "Code-Fight";
    let test_prefix_loc = "/test";

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.reqopt("l", "language", "(required) select your language.", "[ python | scala ]");
    opts.reqopt("s", "subject", "(required) set subject name to submit", "e.g. word_count");
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

                           lang_test::run_spark_test(&language, &user, &subject); 

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
