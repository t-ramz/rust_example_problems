use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // args function here will panic at invalid unicode
    let args: Vec<String> = env::args().collect();  // std lib iterator of arguments
                                                    // iterators produce a series of values
                                                    // collect will turn the values into collection
    let bin_arg: &String = &args[0];
    let passed_args: &[String] = &args[1..args.len()];

    let config = Config::new(passed_args).unwrap_or_else(|err| {    // use closure
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {   // if let more sensible because we don't need value
        println!("Application error occurred: {}", e);
        process::exit(1);
    }
}
