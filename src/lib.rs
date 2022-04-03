use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    return Ok(());
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query: String = args[1].clone();
        let filename: String = args[2].clone();
        return Ok(Config { query, filename });
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query: String = query.to_lowercase();
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_one_result() {
        let query: &str = "nOthiNG";
        let contents: &str = "Nothing Else Metters";

        assert_eq!(vec!["Nothing Else Metters"], search(query, contents));
    }

    #[test]
    fn should_not_find_any_result() {
        let query: &str = "patience";
        let contents: &str = "Nothing Else Metters";

        assert_ne!(vec!["Nothing Else Metters"], search(query, contents));
    }
}
