use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::io::ErrorKind;
use std::error::Error;
use std::net::IpAddr;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    // panic!("crash and burn");

    let greeting_file_result = File::open("hello.txt");

    let greeting_file_one = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    println!("File 1: {greeting_file_one:?}");

    let greeting_file_two = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    println!("File 2: {greeting_file_two:?}");

    // let greeting_file_three = File::open("hello_three.txt")
    //     .expect("hello_three.txt should be included in this project");

    // println!("File 3: {greeting_file_three:?}");

    let read_file_one = read_username_from_file_one();
    println!("Read File 1: {read_file_one:?}");
    let read_file_two = read_username_from_file_two();
    println!("Read File 2: {read_file_two:?}");
    let read_file_three = read_username_from_file_three();
    println!("Read File 3: {read_file_three:?}");
    let read_file_four = read_username_from_file_four();
    println!("Read File 4: {read_file_four:?}");

    let last_char = last_char_of_first_line("john.doe");
    println!("Last char: {last_char:?}");

    // let greeting_file_last = File::open("hello.txt")?; //  Attempting to use the ? in the main function that returns () won’t compile.

    let greeting_file_last = File::open("hello.txt")?; // Changing main to return Result<(), E> allows the use of the ? operator on Result values.
    println!("Updated main return type: {greeting_file_last:?}");

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");

    println!("Panic Error 1: {home:?}");

    Ok(())
}

fn read_username_from_file_one() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_two() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_three() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_four() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}