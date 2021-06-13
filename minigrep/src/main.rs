use std::{env, fs::File, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_name = &args[2];
    println!("{:?}", query);
    println!("{:?}", file_name);

    let mut file = File::open(file_name).expect("file not found!");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file!");
    println!("{:?}", contents);
}
