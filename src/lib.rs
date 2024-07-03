use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    let str_query = config.query.to_str().unwrap();

    let results = if config.ignore_case {
        insensitive_search(str_query, &contents)
    } else {
        search(str_query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    query: OsString,
    file_path: OsString,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[OsString]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments: \n Please use a -- <SearchTerm> <File>");
        }

        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut grep_vec: Vec<&'a str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            grep_vec.push(line);
        }
    }
    grep_vec
}

pub fn insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut in_grep_vec: Vec<&'a str> = Vec::new();
    let search_term = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&search_term) {
            in_grep_vec.push(line)
        }
    }
    in_grep_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn matching_insensitive_cases() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            insensitive_search(query, contents)
        );
    }
}
