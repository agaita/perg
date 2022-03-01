use std::env;
use std::process;
use perg::config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = config::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = perg::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

