mod lang;
pub mod parser;

pub fn run_spark_test(language: &str, user: &str, subject: &str, solution: &str) -> parser::ResponseMetrics {
    match language {
        "python" => {
            let lang_py: lang::Python = lang::Lang::new(user.to_string(), subject.to_string());
            let text = lang::Lang::run_test(&lang_py, solution);
            parser::parse_py_rsp(&text)
        },
        "scala" => {
            let lang_scala: lang::Scala = lang::Lang::new(user.to_string(), subject.to_string());
            let text = lang::Lang::run_test(&lang_scala, solution);
            parser::parse_scala_rsp(&text)
        },
        _ => panic!("Invalid language"),
    }
}
