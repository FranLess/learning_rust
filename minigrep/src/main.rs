use std::{env, error::Error, fs, process};

use minigrep::{run, Config};

fn main() {
    // read args from command line execution
    // let args: Vec<String> = env::args().collect(); // notice that this will panic if got invalid unicode
                                                   // env::args_os would be a better way to handle args
                                                   // but it is more complex so will stay this way

    // dbg!(args); // -- for deb purposes
    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });
    //V2
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for: {}", config.query);
    println!("In the file: {}", config.file_path);
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1)
    }
}
