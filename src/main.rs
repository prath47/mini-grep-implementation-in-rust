use std::{env, process};

use mini_grep_impl::{Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Arguments Are: {:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Query is: {}", config.query);
    println!("Filename is: {}", config.filename);

    if let Err(e) = mini_grep_impl::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}