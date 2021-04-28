use std::env;
use std::process;

use learn_rust::Config;

static EXPECTED_USAGE: &str = "Expected usage: learn_rust <search_query> <file_name>";

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Failed to parse arguments: {}\n{}", err, EXPECTED_USAGE);
        process::exit(1);
    });

    if let Err(e) = learn_rust::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

