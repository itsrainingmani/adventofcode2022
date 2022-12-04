use std::fs::File;
use std::{
    error::Error,
    io::{BufRead, BufReader},
};

enum MoveType {
    Rock,
    Paper,
    Scissor,
}

struct Move {
    kind: MoveType,
    score: u8,
}

impl Move {
    fn evaluate_outcome(m1: &Move, outcome: &str) -> Move {
        match outcome {
            "X" => match m1.kind {
                // Lose
                MoveType::Rock => Move {
                    kind: MoveType::Scissor,
                    score: 3,
                },
                MoveType::Paper => Move {
                    kind: MoveType::Rock,
                    score: 1,
                },
                MoveType::Scissor => Move {
                    kind: MoveType::Paper,
                    score: 2,
                },
            },
            "Y" => match m1.kind {
                // Draw
                MoveType::Rock => Move {
                    kind: MoveType::Rock,
                    score: 1,
                },
                MoveType::Paper => Move {
                    kind: MoveType::Paper,
                    score: 2,
                },
                MoveType::Scissor => Move {
                    kind: MoveType::Scissor,
                    score: 3,
                },
            },
            "Z" => match m1.kind {
                // Win
                MoveType::Rock => Move {
                    kind: MoveType::Paper,
                    score: 2,
                },
                MoveType::Paper => Move {
                    kind: MoveType::Scissor,
                    score: 3,
                },
                MoveType::Scissor => Move {
                    kind: MoveType::Rock,
                    score: 1,
                },
            },
            _ => panic!("invalid move"),
        }
    }
}

struct Round {
    score: u8,
}

impl Round {
    fn new(moves: String) -> Round {
        let (p1move, p2outcome) = moves.split_once(' ').expect("should only be two moves");

        let player1_move = match p1move {
            "A" => Move {
                kind: MoveType::Rock,
                score: 1,
            },
            "B" => Move {
                kind: MoveType::Paper,
                score: 2,
            },
            "C" => Move {
                kind: MoveType::Scissor,
                score: 3,
            },
            _ => panic!("invalid move"),
        };

        let player2_move = Move::evaluate_outcome(&player1_move, p2outcome);

        let score: u8 = match player1_move.kind {
            MoveType::Rock => match player2_move.kind {
                MoveType::Rock => 3u8,
                MoveType::Paper => 6u8,
                MoveType::Scissor => 0u8,
            },
            MoveType::Paper => match player2_move.kind {
                MoveType::Rock => 0u8,
                MoveType::Paper => 3u8,
                MoveType::Scissor => 6u8,
            },
            MoveType::Scissor => match player2_move.kind {
                MoveType::Rock => 6u8,
                MoveType::Paper => 0u8,
                MoveType::Scissor => 3u8,
            },
        } + player2_move.score;

        Round { score }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Advent of Code 2022 - Day 2");

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut cur_round: Round;
    let mut total_score: u64 = 0;

    for line in reader.lines().map(|l| l.unwrap()) {
        cur_round = Round::new(line);
        total_score += cur_round.score as u64;
    }

    println!("{}", total_score);

    Ok(())
}
