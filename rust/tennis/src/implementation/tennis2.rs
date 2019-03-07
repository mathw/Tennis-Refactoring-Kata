use crate::tennis::Tennis;
use std::str::FromStr;

struct TennisPlayer {
    name: String,
    score: u8,
}

impl TennisPlayer {
    fn add_score_if_name(&mut self, name: &str) {
        if self.name == name.to_owned() {
            self.score += 1;
        }
    }

    fn describe_score(&self) -> &'static str {
        match self.score {
            0 => "Love",
            1 => "Fifteen",
            2 => "Thirty",
            3 => "Forty",
            _ => "",
        }
    }

    fn has_deuce_score(&self) -> bool {
        self.score > 2
    }

    fn has_potential_win(&self) -> bool {
        self.score > 3
    }
}

pub struct TennisGame {
    player1: TennisPlayer,
    player2: TennisPlayer,
}

impl TennisGame {
    pub fn new(player1_name: &str, player2_name: &str) -> TennisGame {
        TennisGame {
            player1: TennisPlayer {
                name: player1_name.into(),
                score: 0,
            },
            player2: TennisPlayer {
                name: player2_name.into(),
                score: 0,
            },
        }
    }

    fn is_draw(&self) -> bool {
        self.player1.score == self.player2.score
    }

    fn is_win_state(&self) -> bool {
        self.player1.has_potential_win() && self.player1.score > self.player2.score + 1
            || self.player2.has_potential_win() && self.player2.score > self.player1.score + 1
    }

    fn is_deuce_state(&self) -> bool {
        self.player1.has_deuce_score() && self.player2.has_deuce_score()
    }
}

impl Tennis for TennisGame {
    fn won_point(&mut self, player_name: &str) {
        self.player1.add_score_if_name(player_name);
        self.player2.add_score_if_name(player_name);
    }

    fn get_score(&self) -> String {
        if self.is_win_state() {
            if self.player1.score > self.player2.score {
                format!("Win for {}", self.player1.name)
            } else if self.player2.score > self.player1.score {
                format!("Win for {}", self.player2.name)
            } else {
                panic!("Impossible game state")
            }
        } else if self.is_deuce_state() {
            if self.player1.score == self.player2.score {
                "Deuce".into()
            } else if self.player1.score > self.player2.score {
                format!("Advantage {}", self.player1.name)
            } else {
                format!("Advantage {}", self.player2.name)
            }
        } else if self.is_draw() {
            format!("{}-All", self.player1.describe_score())
        } else {
            format!(
                "{}-{}",
                self.player1.describe_score(),
                self.player2.describe_score()
            )
        }
    }
}
