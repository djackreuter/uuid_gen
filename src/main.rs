use std::env;
use std::process;
//use std::error::Error;
use uuid_gen::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });

    if let Err(e) = uuid_gen::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}
