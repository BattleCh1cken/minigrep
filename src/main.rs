use minigrep::Config;
use std::{env, process};
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("There was an error: {err}");
        process::exit(1);
    });
    println!("{:?}", config);
    if let Err(e) = Config::run(config) {
        println!("There was an error: {e}")
    }
}
