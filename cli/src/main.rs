use std::env::{self, args};
use std::error::Error;
use std::fs;
use std::process;
// mini grep

fn main() {
    let arguments: Vec<String> = args().collect();

    let config = Config::new(&arguments).unwrap_or_else(|err| {
        println!("failed to parse args: {:?}", err);
        process::exit(1);
    });
    run(config).unwrap_or_else(|err|{
        println!("failed to read file {:?}", err);
        process::exit(1)
    });
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string(config.file_name)?;
    println!("{:?}", file);
    Ok(())
}

struct Config {
    word: String,
    file_name: String,
}

impl Config {
    fn new(config_str: &[String]) -> Result<Config, &str> {
        if config_str.len() < 3 {
            return Err("NOT ENOUGH ARUMENTS.");
        }

        let word = config_str[1].clone();
        let file_name = config_str[2].clone();

        Ok(Config { word, file_name })
    }
}
