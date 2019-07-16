use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
//    fn new(args: &[String]) -> Config {
//        // 7. improving the error message
//        if args.len() < 3 {
//            panic!("not enough arguments");
//        }
//        let query = args[1].clone();
//        let filename = args[2].clone();
//
//        Config {query, filename}
//    }

    // 8. return a Result from new instead of calling panic!

    //    pub fn new(args: &[String]) -> Result<Config, &'static str> {
//        if args.len() < 3 {
//            return Err("not enough arguments");
//        }
//
//        let query = args[1].clone();
//        let filename = args[2].clone();
//
//        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
//
//
//        Ok(Config { query, filename , case_sensitive})
//    }

    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

// 11. returning error from run function
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

//    println!("With contents:\n{}", contents);
    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//    let mut results = Vec::new();
//
//    for line in contents.lines() {
//        if line.contains(query) {
//            results.push(line);
//        }
//    }
//
//    return results;

    contents.lines().filter(|line| {
        line.contains(query)
    }).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//    let mut results = Vec::new();
//
//    let query = query.to_lowercase();
//
//    for line in contents.lines() {
//        if line.to_lowercase().contains(&query) {
//            results.push(line);
//        }
//    }
//
//    return results;

    contents.lines().filter(|line| {
        line.to_lowercase().contains(&query.to_lowercase())
    }).collect()
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

        assert_eq!(vec!["safe, fast, productive."],
                   search(query, contents));
    }

    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."],
                   search_case_insensitive(query, contents));
    }
}
