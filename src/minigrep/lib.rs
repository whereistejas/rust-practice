use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    for line in search(&config.searchstring, &contents, config.case_insensitive) {
        println!("Line: {}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, case_insensitive: bool) -> Vec<&'a str> {
    let searchstring = String::from(query);

    if case_insensitive {
        contents
            .lines()
            .filter(|line| line.contains(&searchstring))
            .collect()
    } else {
        contents
            .lines()
            .filter(|line| line.to_lowercase().contains(&searchstring.to_lowercase()))
            .collect()
    }
}

pub struct Config {
    pub searchstring: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new<'a>(mut args: env::Args) -> Result<Config, &'a str> {
        if args.len() < 3 {
            Err("Too few input arguments")
        } else {
            args.next();
            let case_insensitive = env::var("CASE_INSENSITIVE").is_err();
            let searchstring = args.next().unwrap();
            let filename = args.next().unwrap();

            Ok(Config {
                searchstring,
                filename,
                case_insensitive,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "luke";
        let contents = "\
the force is strong with him,
my name is luke,
the force is real";

        assert_eq!(vec!["my name is luke,"], search(query, contents, false));
    }

    #[test]
    fn search_case_sensitive() {
        let query = "Luke";
        let contents = "\
the force is strong with him,
my name is Luke,
the force is real";

        assert_eq!(vec!["my name is Luke,"], search(query, contents, true));
    }
    #[test]
    fn search_case_insensitive() {
        let query = "LuKe";
        let contents = "\
the force is strong with him,
my name is luke,
the force is real";

        assert_eq!(vec!["my name is luke,"], search(query, contents, false));
    }
}
