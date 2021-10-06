use rand::{random, thread_rng, Rng};
use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};

pub fn gen_csv(file_path: &str) -> anyhow::Result<()> {
    let lines = 100000;
    let timestamp = "2020/01/01 12:00";
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
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
        let id: u8 = random();
        let player_id = format!("player{}", id);
        let score = rng.gen_range(0..10000);
        let record = format!("{},{},{}\n", timestamp, player_id, score);
        file.write_all(record.as_bytes())?;
    }

    Ok(())
}
