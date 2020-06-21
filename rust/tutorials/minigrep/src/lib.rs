//! # My own minigrep
//!
//! `minigrep` is a crate part of the official learn-rust book.
//! This is the result of following the tutorial.
//!
use std::fs;
use std::env;
use std::error::Error;



pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}


impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(a) => a,
            None => return Err("Please enter the query arg!")
        };

        let filename = match args.next() {
            Some(a) => a,
            None => return Err("Please enter the filename arg!")
        };

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


/// Filters lines that match the pattern.
///
/// # Examples
///
/// ```
/// 
/// let query = "duct";
/// let contents = "\
///Rust:
///safe, fast, productive.
///Pick three.
///Duct tape.";
/// assert_eq!(vec!["safe, fast, productive."],
///            minigrep::search(query, contents));
/// ```

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .split("\n") 
        .filter(|x| x.contains(query))
        .collect()
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
