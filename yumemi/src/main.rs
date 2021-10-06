mod gen_csv;

use std::io;

use csv::{Reader, Writer};
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

#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq)]
struct GameScoreMean {
    score: u32,
    player_id: String,
}

#[derive(Debug, Clone)]
struct GameScoreRank {
    rank: u32,
    player_id: String,
    mean_score: u32,
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

    fn to_ranks(&self, limit: u32) -> Vec<GameScoreRank> {
        let mut game_score_ranks: Vec<GameScoreRank> = Vec::new();
        let mut means = self.to_means();

        // sort means order by desc
        means.sort();
        means.reverse();

        // push first place
        let first = means.first().unwrap();
        let game_score_rank = GameScoreRank {
            rank: 1,
            player_id: first.clone().player_id,
            mean_score: first.score,
        };
        game_score_ranks.push(game_score_rank);

        for mean in means.windows(2) {
            // current starts 2nd place
            if let [previous, current] = mean {
                let rank_clone = game_score_ranks.clone();
                let bottom = rank_clone.last().unwrap();

                // break when reached limit
                if bottom.rank == limit {
                    break;
                }

                let rank = if previous.score == current.score {
                    bottom.rank
                } else {
                    game_score_ranks.len() as u32 + 1
                };
                let game_score_rank = GameScoreRank {
                    rank,
                    player_id: current.clone().player_id,
                    mean_score: current.score,
                };
                game_score_ranks.push(game_score_rank);
            }
        }
        game_score_ranks
    }

    fn to_means(&self) -> Vec<GameScoreMean> {
        let mut game_score_means: Vec<GameScoreMean> = Vec::new();
        for score in &self.0 {
            let mean_score = score.sum / score.play_count;
            let game_score_mean = GameScoreMean {
                player_id: score.clone().player_id,
                score: mean_score,
            };
            game_score_means.push(game_score_mean);
        }
        game_score_means
    }
}

fn main() {
    let file_path = "./data/game_score_log.csv";

    // generate csv if not exist
    // create_dir_all("data").expect("failed to create directory");
    // gen_csv(file_path).expect("failed to generate csv");
    run(file_path).unwrap();
}

fn run(file_path: &str) -> anyhow::Result<()> {
    let mut rdr = Reader::from_path(file_path).unwrap();
    let mut game_score_summary = GameScoreSummary::new();

    for result in rdr.deserialize() {
        let record: GameScoreLogRecord = result.unwrap();
        let score = GameScore::from_record(record);
        game_score_summary.upsert(score);
    }

    // // debug
    // for score in &game_score_summary.0 {
    //     println!("{:?}", score);
    // }
    // let mut means = game_score_summary.to_means();
    // for mean in &means {
    //     println!("{:?}", mean);
    // }
    // means.sort();
    // means.reverse();
    // for mean in means {
    //     println!("sorted: {:?}", mean);
    // }
    // let ranks = game_score_summary.to_ranks(10);
    // for rank in ranks {
    //     println!("{:?}", rank);
    // }
    // // debug

    let mut wtr = Writer::from_writer(io::stdout());
    // write header
    wtr.write_record(&["rank", "player_id", "mean_score"])
        .unwrap();
    game_score_summary
        .to_ranks(10)
        .into_iter()
        .for_each(|score_rank| {
            wtr.write_record(&[
                score_rank.rank.to_string(),
                score_rank.player_id,
                score_rank.mean_score.to_string(),
            ])
            .unwrap();
        });
    wtr.flush().unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_csv() {}
}
