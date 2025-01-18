#![allow(unused_variables)]
use std::{env, process};

use tmr::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    println!("Timer: {} second", config.duration);

    if let Err(e) = tmr::run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}
