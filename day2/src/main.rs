use std::{
    error::Error,
    io::{BufRead, BufReader},
};
use std::{fmt, fs::File};

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
    fn new(m: char) -> Move {
        let kind = match m {
            'A' | 'X' => MoveType::Rock,
            'B' | 'Y' => MoveType::Paper,
            'C' | 'Z' => MoveType::Scissor,
            _ => panic!("Invalid Move"),
        };

        let score: u8 = match kind {
            MoveType::Rock => 1u8,
            MoveType::Paper => 2u8,
            MoveType::Scissor => 3u8,
        };

        Move { kind, score }
    }
}

struct Round {
    P1Move: Move,
    P2Move: Move,
    score: u8,
}

impl Round {
    fn new(moves: String) -> Round {
        let (p1move, p2move) = moves.split_once(' ').expect("should only be two moves");

        let player1_move: Move = Move::new(p1move.chars().next().expect("first index"));
        let player2_move: Move = Move::new(p2move.chars().next().expect("first index"));

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

        Round {
            P1Move: player1_move,
            P2Move: player2_move,
            score,
        }
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
