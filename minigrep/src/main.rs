use std::env;
use std::process;

use minigrep::Config;

fn main() {

    //1. parse args

//    let args: Vec<String> = env::args().collect();

//    println!("{:?}", args);

    // 2. save args to variant

//    let query = &args[1];
//    let filename = &args[2];
//    println!("Searching for {}", query);
//    println!("In file {}", filename);

    // 3. reading a file

//    let contents = fs::read_to_string("poem.txt")
//        .expect("Something went wrong reading the file");
//
//    println!("With contents:\n{}", contents);

    // 4. extracting the argument parser

//    let (query, filename) = parse_config(&args);

//    println!("Searching for {}", query);
//    println!("In file {}", filename);

    // 5. grouping configuration values

//    let config = parse_config(&args);


    // 6. create constructor for config

//    let config = Config::new(&args);

    // 9. Handling Errors

//    let config = Config::new(&args)
//        .unwrap_or_else(|err| {
////            println!("Problem parsing arguments: {}", err);
//            eprintln!("Problem parsing arguments: {}", err);
//            process::exit(1);
//        });

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
//    println!("Searching for {}", config.query);
//    println!("In file {}", config.filename);

//    run(config);

    // 12. handle error returned from run
//    if let Err(e) = run(config) {
//        println!("Application error: {}", e);
//        process::exit(1);
//    };

    if let Err(e) = minigrep::run(config) {
//        println!("Application error: {}", e);
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

// 10. extracting logic from main
//fn run(config: Config) {
//    let contents = fs::read_to_string(config.filename)
//        .expect("Something went wrong reading the file");
//
//    println!("With contents:\n{}", contents);
//}


//fn parse_config(args: &[String]) -> (&str, &str) {
//    let query = &args[1];
//    let filename = &args[2];
//
//    (query, filename)
//}


//fn parse_config(args: &[String]) -> Config {
//    let query = args[1].clone();
//    let filename = args[2].clone();
//
//    Config { query, filename }
//}


