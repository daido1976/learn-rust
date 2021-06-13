extern crate minigrep;
use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match minigrep::run(config) {
        Ok(_) => (),
        Err(e) => {
            println!("Application error: {}", e);
            process::exit(1);
        }
    }
}
