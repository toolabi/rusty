use std::env::{self, args};
use std::process;
use minigrep::Config;
// mini grep

fn main() {
    let arguments: Vec<String> = args().collect();

    let config = Config::new(&arguments).unwrap_or_else(|err| {
        eprintln!("failed to parse args: {:?}", err);
        process::exit(1);
    });
    minigrep::run(&config).unwrap_or_else(|err|{
        eprintln!("failed to read file {:?}", err);
        process::exit(1)
    });
}

