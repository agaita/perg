use std::env;

#[derive(PartialEq, Debug)]
pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
    pub case_sensitive: bool,
}

impl<'a> Config<'a> {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        let query = &args[1];
        let filename = &args[2];

        Ok(Config { query, filename, case_sensitive })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_with_not_enough_arguments() {
        let mut args = vec![];
        assert_eq!(Config::new(&args), Err("not enough arguments"));

        args.push(String::new());
        assert_eq!(Config::new(&args), Err("not enough arguments"));

        args.push(String::new());
        assert_eq!(Config::new(&args), Err("not enough arguments"));

        args.push(String::new());
        assert!(Config::new(&args).is_ok());
    }

    #[test]
    fn new() {
        let args = vec![
            String::from("target/debug/perg"),
            String::from("query"),
            String::from("filename"),
        ];

        let expected_config = Config {
            query: "query",
            filename: "filename",
            case_sensitive: true,
        };

        assert_eq!(Config::new(&args), Ok(expected_config));
    }
}
