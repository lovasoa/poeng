use poeng_server::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error creating Config: {}", err);
        println!("Usage: poeng_server url");
        process::exit(1);
    });

    poeng_server::run(config);
}
