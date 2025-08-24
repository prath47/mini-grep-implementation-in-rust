use std::{env, error::Error, fs};


pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new (args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = match env::var("CASE_SENSITIVE") {
            Ok(t) => {
                if t == "true" {
                    true
                } else {
                    false
                }
            },
            Err(_) => {
                true
            }
        };

        println!("Case Sensitive {}", case_sensitive);
        Ok(Config { query, filename, case_sensitive})
    }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {

    let content = fs::read_to_string(config.filename)?;
    // println!("The content of the file is: {} \n", contents);

    let result = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a> (query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for lines in content.lines() {
        if lines.contains(query) {
            results.push(lines);
        }
    }

    results
} 

pub fn search_case_insensitive<'a> (query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for lines in content.lines() {
        if lines.to_lowercase().contains(&query.to_lowercase()) {
            results.push(lines);
        }
    }

    results
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
Duct Tape";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["Rust", "Trust Me"], search_case_insensitive(query, contents));
    }
}