use std::env;
use std::process;

use nbabel;
use nbabel::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = nbabel::run(config) {
        println!("Runtime error: {}", e);

        process::exit(1);
    }
}
