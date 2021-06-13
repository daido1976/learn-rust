use std::{env, error::Error, fs::File, io::Read, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match run(config) {
        Ok(_) => (),
        Err(e) => {
            println!("Application error: {}", e);
            process::exit(1);
        }
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(config.file_name)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    println!("{:?}", contents);

    Ok(())
}

struct Config {
    query: String,
    file_name: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }
        let query = args[1].clone();
        let file_name = args[2].clone();

        Ok(Config { query, file_name })
    }
}
