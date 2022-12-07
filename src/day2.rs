use std::{
    error::Error,
    fs::File,
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
    fn both_moves(moves: String) -> Round {
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

        Round { score }
    }

    fn single_move_outcome(moves: String) -> Round {
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

fn rock_paper_scissors() -> Result<(), Box<dyn Error>> {
    let file = File::open("inputs/day2.txt")?;
    let reader = BufReader::new(file);
    let mut cur_round: Round;
    let mut total_score: u64 = 0;

    for line in reader.lines().map(|l| l.unwrap()) {
        cur_round = Round::both_moves(line);
        total_score += cur_round.score as u64;
    }

    println!("\tPart 1A - {}", total_score);
    Ok(())
}

fn reverse_rps() -> Result<(), Box<dyn Error>> {
    let file = File::open("inputs/day2.txt")?;
    let reader = BufReader::new(file);
    let mut cur_round: Round;
    let mut total_score: u64 = 0;

    for line in reader.lines().map(|l| l.unwrap()) {
        cur_round = Round::single_move_outcome(line);
        total_score += cur_round.score as u64;
    }

    println!("\tPart 1B - {}", total_score);
    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("\nDay 2");

    rock_paper_scissors()?;
    reverse_rps()?;

    Ok(())
}
