use std::fs;
use std::env;
use std::error::Error;


pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}


impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {query, filename, case_sensitive})
    }
}


pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(cfg.filename)?;

    let results = if cfg.case_sensitive {
        search(&cfg.query, &contents)
    } else {
        search_case_insensitive(&cfg.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut vec = Vec::new();

    for s in contents.split("\n") {
        match s.find(query) {
            Some(_) => vec.push(s),
            None => ()
        }
    }
    vec
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut vec = Vec::new();

    let query = query.to_lowercase();

    for s in contents.split("\n") {
        match s.to_lowercase().find(&query) {
            Some(_) => vec.push(s),
            None => ()
        }
    }
    vec
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() {
        let mut v: Vec<String> = Vec::new();
        v.push("cli name".to_string());
        v.push("query".to_string());
        v.push("filename".to_string());

        let cfg = Config::new(&v).unwrap();
        assert_eq!(cfg.query, "query");
        assert_eq!(cfg.filename, "filename");
    }

    #[test]
    fn test_config_wrong_number_args() -> () {
        let mut v: Vec<String> = Vec::new();
        v.push("cli name".to_string());
        v.push("query".to_string());
        let cfg = Config::new(&v);
        assert!(cfg.is_err());
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
