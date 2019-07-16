use std::fs::File;
use std::io::{ErrorKind, Read};
use std::io;
use std::fs;

fn main() {
//    handle_result_using_match();
//    matching_different_error();
//    deal_error_using_closure();
//    read_username_from_file();

//    shortcut_propagating_error();

//    chaining_calls_propagating_shortcut();

    shortcut_using_fs_read_to_string();

}

fn handle_result_using_match() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("problem opening the file! {:?}", error);
        }
    };
}

fn matching_different_error() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem create the file! {:?}", e),
            },
            other_error => panic!("Problem open the file! {:?}", other_error),
        }
    };
}

fn deal_error_using_closure() {

    let f = File::open("hello.txt").unwrap_or_else(|error| {

        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|e|{
                panic!("Problem create the file! {:?}", e);
            })
        } else {
            panic!("Problem open the file! {:?}", error);
        }

    });

}

fn deal_error_using_unwrap() {
    let f = File::open("hello.txt").unwrap();
}

fn deal_error_using_expect() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

// propagating errors

fn read_username_from_file() -> Result<String, io::Error> {

    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }

}

fn shortcut_propagating_error() -> Result<String, io::Error>{

    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(s)

}

fn chaining_calls_propagating_shortcut() -> Result<String, io::Error> {

    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn shortcut_using_fs_read_to_string() -> Result<String, io::Error>{
    fs::read_to_string("hello.txt")
}
