use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_name);

    let contents = fs::read_to_string(config.file_name).expect("Error reading the file");
    println!("Contents:\n{} ", contents);
}

struct Config {
    query: String,
    file_name: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[0].clone();
        let file_name = args[1].clone();

        Config { query, file_name }
    }
}
