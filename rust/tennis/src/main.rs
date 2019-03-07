use crate::implementation::tennis1;
use crate::tennis::Tennis;

mod implementation;
mod tennis;

fn main() {
    println!("Run 'cargo test' to run the test suite.");
}

#[cfg(test)]
fn test_specific_score(game: &mut impl Tennis, p1: u8, p2: u8, rendered: &str) {
    if p1 < p2 {
        for _ in 0..p1 {
            game.won_point("player1");
            game.won_point("player2");
        }
        for _ in 0..(p2 - p1) {
            game.won_point("player2");
        }
    }
    if p2 < p1 {
        for _ in 0..p2 {
            game.won_point("player1");
            game.won_point("player2");
        }
        for _ in 0..(p1 - p2) {
            game.won_point("player1");
        }
    }
    if p1 == p2 {
        for _ in 0..p1 {
            game.won_point("player1");
            game.won_point("player2");
        }
    }

    assert_eq!(
        game.get_score(),
        rendered.to_owned(),
        "{}-{} should be {}",
        p1,
        p2,
        rendered
    );
}

#[cfg(test)]
fn test_realistic_tennis_game(game: &mut impl Tennis) {
    let points = vec![
        "player1", "player1", "player2", "player2", "player1", "player1",
    ];
    let expected_scores = vec![
        "Fifteen-Love",
        "Thirty-Love",
        "Thirty-Fifteen",
        "Thirty-All",
        "Forty-Thirty",
        "Win for player1",
    ];

    for (point, expected_score) in points.iter().zip(expected_scores) {
        game.won_point(point);
        assert_eq!(game.get_score(), expected_score.to_owned());
    }
}

#[cfg(test)]
fn specific_scores() -> Vec<(u8, u8, &'static str)> {
    vec![
        (0, 0, "Love-All"),
        (1, 1, "Fifteen-All"),
        (2, 2, "Thirty-All"),
        (3, 3, "Deuce"),
        (4, 4, "Deuce"),
        (1, 0, "Fifteen-Love"),
        (0, 1, "Love-Fifteen"),
        (2, 0, "Thirty-Love"),
        (0, 2, "Love-Thirty"),
        (3, 0, "Forty-Love"),
        (0, 3, "Love-Forty"),
        (4, 0, "Win for player1"),
        (0, 4, "Win for player2"),
        (2, 1, "Thirty-Fifteen"),
        (1, 2, "Fifteen-Thirty"),
        (3, 1, "Forty-Fifteen"),
        (1, 3, "Fifteen-Forty"),
        (4, 1, "Win for player1"),
        (1, 4, "Win for player2"),
        (3, 2, "Forty-Thirty"),
        (2, 3, "Thirty-Forty"),
        (4, 2, "Win for player1"),
        (2, 4, "Win for player2"),
        (4, 3, "Advantage player1"),
        (3, 4, "Advantage player2"),
        (5, 4, "Advantage player1"),
        (4, 5, "Advantage player2"),
        (15, 14, "Advantage player1"),
        (14, 15, "Advantage player2"),
        (6, 4, "Win for player1"),
        (4, 6, "Win for player2"),
        (16, 14, "Win for player1"),
        (14, 16, "Win for player2"),
    ]
}

#[test]
fn test_tennis1() {
    let mut tennis1 = tennis1::TennisGame::new("player1", "player2");
    test_realistic_tennis_game(&mut tennis1);
    for (p1, p2, result) in specific_scores() {
        test_specific_score(
            &mut tennis1::TennisGame::new("player1", "player2"),
            p1,
            p2,
            result,
        );
    }
}
