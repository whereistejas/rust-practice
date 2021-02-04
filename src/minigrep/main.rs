use std::{env, process};

use greprs::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Too few arguments were provided: {}", err);
        process::exit(1);
    });

    if let Err(e) = greprs::run(&config) {
        eprintln!("Error: {} ", e);
        process::exit(1);
    }
}
