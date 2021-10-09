use csv::{Reader, Writer};
use std::fs::File;
use std::io::{self, BufReader};
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
// TODO: struct GameScoreSummary(HashMap<String, GameScore>); にする
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
        // TODO: この find で全件捜査して時間かかってるので HashMap での key アクセスに変える
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

pub fn run(file_path: &str) -> anyhow::Result<()> {
    let file = File::open(file_path)?;
    let rdr = BufReader::new(file);
    let mut rdr = Reader::from_reader(rdr);
    let mut game_score_summary = GameScoreSummary::new();

    for result in rdr.deserialize() {
        let record: GameScoreLogRecord = result.unwrap();
        let score = GameScore::from_record(record);
        game_score_summary.upsert(score);
    }

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
