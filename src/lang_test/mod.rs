mod lang;

pub fn run_spark_test(language: &str, user: &str, subject: &str) -> String {
    match language {
        "python" => {
            let lang_py: lang::Python = lang::Lang::new(user.to_string(), subject.to_string());
            lang::Lang::run_test(&lang_py)
        },
        "scala" => {
            let lang_scala: lang::Scala = lang::Lang::new(user.to_string(), subject.to_string());
            lang::Lang::run_test(&lang_scala)
        },
        _ => panic!("Invalid language"),
    }
}
