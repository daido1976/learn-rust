mod gen_csv;

use csv::Reader;
use gen_csv::gen_csv;

// create_timestamp,player_id,score
type GameScoreLogRecord = (String, String, u32);

#[derive(Debug, Clone)]
struct GameScore {
    player_id: String,
    sum: u32,
    play_count: u32,
}

impl GameScore {
    fn from_record(record: GameScoreLogRecord) -> Self {
        Self {
            player_id: record.1.clone(),
            sum: record.2,
            play_count: 1,
        }
    }
}

#[derive(Debug)]
struct GameScoreSummary(Vec<GameScore>);

impl GameScoreSummary {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn upsert(&mut self, new_score: GameScore) {
        if let Some(score) = self.find(new_score.clone().player_id) {
            // update
            score.sum += new_score.sum;
            score.play_count += new_score.play_count;
        } else {
            // insert
            self.insert(new_score);
        }
    }

    fn find(&mut self, player_id: String) -> Option<&mut GameScore> {
        self.0.iter_mut().find(|score| score.player_id == player_id)
    }

    fn insert(&mut self, new_score: GameScore) {
        self.0.push(new_score);
    }
}

// 1. プレイヤーidと平均スコアのベクタ
// 2. 平均スコアでソートしたベクタ
// 3. 2にrankをつけたベクタのrank10位まで
fn main() {
    let file_path = "./data/game_score_log.csv";

    // generate csv if not exist
    // gen_csv(file_path).unwrap();
    run(file_path).unwrap();
}

fn run(file_path: &str) -> anyhow::Result<()> {
    let mut rdr = Reader::from_path(file_path).unwrap();
    let mut game_score_summary = GameScoreSummary::new();

    for result in rdr.deserialize() {
        let record: GameScoreLogRecord = result.unwrap();
        println!("{:?}", record);
        let score = GameScore::from_record(record);
        game_score_summary.upsert(score);
    }
    for score in game_score_summary.0 {
        println!("{:?}", score);
    }

    // let mut wtr = Writer::from_writer(io::stdout());
    // // write header
    // wtr.write_record(&["rank", "player_id", "mean_score"])
    //     .unwrap();
    // rdr.records().for_each(|result| {
    //     let record = result.unwrap();
    //     wtr.write_record(&["1", record.get(1).unwrap(), record.get(2).unwrap()])
    //         .unwrap();
    // });
    // wtr.flush().unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_csv() {}
}
