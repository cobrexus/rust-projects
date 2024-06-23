use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|e| {
        eprintln!("config error: {e}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("os error: {e}");
        process::exit(1);
    }
}
