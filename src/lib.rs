use std::error::Error;
use std::fs;

/// Config passed in as command line arguments when running the program
///
/// * `query` - search query
/// * `file_path` - path of the file to search in
///
pub struct Config {
    pub query: String,
    pub file_path: String
}

impl Config {
    /// Returns a `Result` containing the `config` if `args` are parsed successfully
    /// else returns an `Err` if length of `args` is less than three.
    ///
    /// # Arguments
    ///
    /// * `args` - A string slice that holds the command line arguments passed
    ///
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

/// Prints the contents of file in the `config` if the result is success
/// else returns an `Error` if unable to parse the file to string.
///
/// # Arguments
///
/// * `config` - A config struct that holds the parsed command line arguments
///
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? will throw an error instead of panicking
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}
