mod game_score;
mod gen_csv;

use game_score::run;
use gen_csv::gen_csv;
use std::{env, fs::create_dir_all, time::Instant};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = "./data/game_score_log.csv";

    if args.len() > 1 && &args[1] == "gen_csv" {
        let lines: i32 = if args.len() > 2 {
            // when passed line count from command line
            args[2].parse().expect("can't parse to number")
        } else {
            // the default value
            100
        };
        create_dir_all("data").expect("failed to create directory");
        gen_csv(file_path, lines).expect("failed to generate csv");
    } else {
        let now = Instant::now();
        run(file_path).expect("failed to run");
        println!(
            "time: {} seconds",
            now.elapsed().as_millis() as f32 / 1000_f32
        );
    }
}
