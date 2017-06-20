mod lang;

pub fn run_spark_test(language: &str, user: &str, subject: &str) {
    match language {
        "python" => {
            let lang_py: lang::Python = lang::Lang::new(user.to_string(), subject.to_string());
            lang::Lang::run_test(&lang_py);
        },
        "scala" => {
        },
        _ => println!("No support for such type language: {}", language),
    }
}
