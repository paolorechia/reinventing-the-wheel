use std::env;
use std::process;

use minigrep::Config;
use minigrep::run;

fn main() {
    let cfg = Config::new(env::args()).unwrap_or_else( |err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(cfg) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
