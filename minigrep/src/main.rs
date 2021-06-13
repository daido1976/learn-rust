use std::{env, fs::File, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    let mut file = File::open(config.file_name).expect("file not found!");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file!");
    println!("{:?}", contents);
}

struct Config {
    query: String,
    file_name: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_name = args[2].clone();

        Config { query, file_name }
    }
}
