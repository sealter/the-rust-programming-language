use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    filename: String,
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

    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// 11. returning error from run function
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With contents:\n{}", contents);

    Ok(())
}
