use std::env;
use std::fs;
use std::process;

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        // exit the program in an error state
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text {}", contents)
}

/// Config passed in as command line arguments when running the program
///
/// * `query` - search query
/// * `file_path` - path of the file to search in
///
struct Config {
    query: String,
    file_path: String
}

impl Config {
    /// Returns a `Result` containing the `config` if `args` are parsed successfully
    /// else returns an `Err` if length of `args` is less than three.
    ///
    /// # Arguments
    ///
    /// * `args` - A string slice that holds the command line arguments passed
    ///
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
