mod lang;
pub mod parser;

pub fn run_spark_test(language: &str, user: &str, subject: &str, solution: &str) -> parser::ResponseMetrics {
    match language {
        "python2" => {
            let lang_py: lang::Python = lang::Lang::new(user.to_string(), subject.to_string());
            let text = lang::Lang::run_test(&lang_py, solution, 2);
            parser::parse_py_rsp(&text)
        },
        "python3" => {
            let lang_py: lang::Python = lang::Lang::new(user.to_string(), subject.to_string());
            let text = lang::Lang::run_test(&lang_py, solution, 3);
            parser::parse_py_rsp(&text)
        },
        "scala" => {
            let lang_scala: lang::Scala = lang::Lang::new(user.to_string(), subject.to_string());
            let text = lang::Lang::run_test(&lang_scala, solution, 2);
            parser::parse_scala_rsp(&text)
        },
        _ => panic!("Invalid language"),
    }
}
