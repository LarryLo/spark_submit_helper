mod lang;

pub fn run_spark_test(language: &str, user: &str, subject: &str, solution: &str) -> String {
    match language {
        "python" => {
            let lang_py: lang::Python = lang::Lang::new(user.to_string(), subject.to_string());
            lang::Lang::run_test(&lang_py, solution)
        },
        "scala" => {
            let lang_scala: lang::Scala = lang::Lang::new(user.to_string(), subject.to_string());
            lang::Lang::run_test(&lang_scala, solution)
        },
        _ => panic!("Invalid language"),
    }
}
