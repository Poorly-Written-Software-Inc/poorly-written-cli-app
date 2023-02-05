use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

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
    /// Returns a config
    ///
    ///
    /// # Arguments
    ///
    /// * `args` - A string slice that holds the command line arguments passed
    ///
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
