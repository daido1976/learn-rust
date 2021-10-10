mod game_score;
mod gen_csv;

use std::{env, fs::create_dir_all, time::Instant};

fn main() {
    let mut args = env::args();
    let file_path = "./data/game_score_log.csv";

    args.next();
    match args.next() {
        None => {
            // when executed `$ cargo run`
            let now = Instant::now();
            game_score::run(file_path).expect("failed to run");
            println!(
                "time: {} seconds",
                now.elapsed().as_millis() as f32 / 1000_f32
            );
        }
        Some(first_arg) if first_arg == "gen_csv" => {
            // when executed `$ cargo run gen_csv <lines>`
            let lines: i32 = args
                .next()
                .unwrap_or_else(|| 100.to_string())
                .parse()
                .expect("failed to parse as a number");
            create_dir_all("data").expect("failed to create directory");
            gen_csv::generate(file_path, lines).expect("failed to generate csv");
        }
        _ => println!("Usage: `cargo run` or `cargo run gen_csv <lines>`"),
    }
}
