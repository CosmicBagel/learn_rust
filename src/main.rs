use std::env;
use std::process;

use alpine_cosmodrome::Config;

static EXPECTED_USAGE: &str = "Expected usage: alpine_cosmodrome <search_query> <file_name>";

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Failed to parse arguments: {}\n{}", err, EXPECTED_USAGE);
        process::exit(1);
    });

    if let Err(e) = alpine_cosmodrome::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

