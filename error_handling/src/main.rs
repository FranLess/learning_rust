use std::{
    error::Error as E,
    fs::File,
    io::{Error, ErrorKind, Read},
};

// The Result type returned in main will allow us to use ? operator
// if the function return Ok(()) it is as we would return a 0 value
// just like in C or C++, where we can explicity see the return 0;
// so if we return another value of Error it will return a diferent integer from 0
// A nonzero value

//The main functio should always return types that implement
// std::process::Termination which contains a function report
// that returns an ExitCode (counsult docs for more info)
fn main() -> Result<(), Box<dyn E>> {
    // Panics and Results
    // panic!("This stops our program to be executed");

    // panic can also hapen when we cause a panic with our code
    let v = vec![1, 123, 3];
    v[99]; // This will panic, even if we don't know it at compile time

    // Result<T, E>
    // the result has the next structure
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    //This code will panic if file doesn't open no matter what
    // let greeting_file_result = File::open("hello.txt");
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // This code will panic if we cannot open the file or if we cannot create the file
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {:?}", e),
            },
            other => {
                panic!("Problem opening the file: {:?}", other);
            }
        },
    };

    //Another way to do the same using closures,
    // closures will be explained in chapter 13
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    Ok(())
}

// A good practice when handling error is Error propagation
// It means that code we write and may result on a error
// should panic, it should return the error to the code
// that called ours
// for example this function will return a Result whereas it be Ok or Err
fn read_username_from_file() -> Result<String, Error> {
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

// ? operator, a shorter way for error propagation
// the ? operator automatically returns Err(e) when it encounters it

fn read_username_from_file2() -> Result<String, Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// The operator ? returns automatically a Err of Result or a None of option
// This could be called in the main function if we add the return type to it
// main function can return Result<(), Box<dyn Error>
// Box is no covered here but we can think about ir as a container for all type of errors
// is it a trait object
