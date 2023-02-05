use std::env;
use std::process;

use poorly_written_cli_app::Config;

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        // exit the program in an error state
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.file_path);

    // not unwrapping because return type is unit
    if let Err(e) = poorly_written_cli_app::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
