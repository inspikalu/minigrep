use std::env::{self};
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
        ) -> Result<Config, &'static str> {
        args.next();
        // if args.len() < 3 {
        //     return Err("not enough arguments");
        // }
        //
        // let query = args[1].clone();
        // let file_path = args[2].clone();



        //TODO: Search up what the .is_ok() does
        //so apparently .is_ok() returns true if the Result is ok
        // .get(3) is used to get the element at the index of 3 it returns an Option enum
        /*
        Below is my implementationf
               let args_exist = match args.get(3) {
                   Some(_) => true,
                   None => false,
               };
               let args_contains_case_sensitive = if args_exist && args[3].clone() == "insensitive"{true} else {false};
               let ignore_case =
                   env::var("IGNORE_CASE").is_ok() || args_contains_case_sensitive;
        */

        //The .map_or() Returns the provided default result (if none), or applies a function to the contained value (if any).
        // let ignore_case = env::var("IGNORE_CASE").is_ok() || args.get(3).map_or(false, |arg| arg == "insensitive");

        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a file path")
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
    // Alternative constructor that panics if not enough arguments are provided
    // This is not recommended for production code, but can be useful for quick prototyping.
    // It is better to handle errors gracefully in production code.
    // This method is used to create a Config instance without error handling.
    // It will panic if the arguments are not sufficient.
    fn _new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Config {
            query,
            file_path,
            ignore_case,
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}")
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results: Vec<&str> = Vec::new();
    // //TODO: Research what the lines do
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(&line);
    //     }
    // }
    // results

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.contains(&query))
        .collect()
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
        )
    }
}
