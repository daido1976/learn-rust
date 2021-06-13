use std::{env, fs::File, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    let (_query, file_name) = parse_config(&args);

    let mut file = File::open(file_name).expect("file not found!");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file!");
    println!("{:?}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_name = &args[2];
    (query, file_name)
}
