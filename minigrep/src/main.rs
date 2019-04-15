extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    //let args: Vec<String> = env::args().collect();

    //struct that holds cmdline args 
    let config = Config::new(env::args()).unwrap_or_else(|err|{
    	eprintln!("Problem with parsing arguments: {}", err);
    	process::exit(1);
    });

    //handle propagated error from minigrep
    if let Err(e) = minigrep::run(config) {
    	eprintln!("Application error: {}", e);
    	process::exit(1);
    }
}