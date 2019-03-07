use crate::tennis::Tennis;
use std::str::FromStr;

pub struct TennisGame {
    player1_name: String,
    player2_name: String,
    player1_score: usize,
    player2_score: usize,
}

impl TennisGame {
    pub fn new(player1_name: &str, player2_name: &str) -> TennisGame {
        TennisGame {
            player1_name: String::from_str(player1_name).unwrap(),
            player2_name: String::from_str(player2_name).unwrap(),
            player1_score: 0,
            player2_score: 0,
        }
    }
}

impl Tennis for TennisGame {
    fn won_point(&mut self, player_name: &str) {
        if String::from_str(player_name).unwrap() == self.player1_name {
            self.player1_score += 1;
        }
        if String::from_str(player_name).unwrap() == self.player2_name {
            self.player2_score += 1;
        }
    }

    fn get_score(&self) -> String {
        let mut result = String::new();

        if self.player1_score > 3 && self.player1_score > self.player2_score + 1 {
            return format!("Win for {}", self.player1_name);
        }

        if self.player2_score > 3 && self.player2_score > self.player1_score + 1 {
            return format!("Win for {}", self.player2_name);
        }

        if self.player1_score == self.player2_score && self.player1_score > 2 {
            return "Deuce".to_owned();
        }

        if self.player1_score > 3 && self.player2_score == self.player1_score - 1 {
            return format!("Advantage {}", self.player1_name);
        }

        if self.player2_score > 3 && self.player1_score == self.player2_score - 1 {
            return format!("Advantage {}", self.player2_name);
        }

        if self.player1_score == 0 {
            result += "Love";
        }
        if self.player1_score == 1 {
            result += "Fifteen";
        }
        if self.player1_score == 2 {
            result += "Thirty";
        }
        if self.player1_score == 3 {
            result += "Forty";
        }

        result += "-";

        if self.player2_score == self.player1_score {
            result += "All";
            return result;
        }

        if self.player2_score == 0 {
            result += "Love";
        }
        if self.player2_score == 1 {
            result += "Fifteen";
        }
        if self.player2_score == 2 {
            result += "Thirty";
        }
        if self.player2_score == 3 {
            result += "Forty";
        }

        result
    }
}
