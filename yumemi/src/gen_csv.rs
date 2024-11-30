use rand::{thread_rng, Rng};
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};

pub fn generate(file_path: &str, lines: i32) -> anyhow::Result<()> {
    let timestamp = "2020/01/01 12:00";
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;

    // clear file contents
    file.set_len(0)?;

    // wrap BufWriter for buffering io
    let mut file = BufWriter::new(file);

    // write header
    file.write_all(b"create_timestamp,player_id,score\n")?;

    // write records
    let mut rng = thread_rng();
    for _ in 0..lines {
        let id = rng.gen_range(0..10000);
        let player_id = format!("player{}", id);
        let score = rng.gen_range(0..10000);
        let record = format!("{},{},{}\n", timestamp, player_id, score);
        file.write_all(record.as_bytes())?;
    }

    Ok(())
}
