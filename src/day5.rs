use std::{
    collections::VecDeque,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("\nDay 5");

    let file = File::open("inputs/day5.txt")?;
    let reader = BufReader::new(file);

    let mut read_cargo: bool = false;
    let mut max_cols: usize = 0;
    let mut cargo_moves: Vec<String> = Vec::new();
    let mut cargo_stacks: Vec<String> = Vec::new();

    for line in reader.lines().map(|l| l.unwrap()).filter(|l| !l.is_empty()) {
        if !read_cargo {
            if line.starts_with(" 1") {
                read_cargo = true;
                max_cols = line
                    .trim()
                    .replace("   ", ",")
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .max()
                    .unwrap();
            } else {
                cargo_stacks.push(line);
            }

            continue;
        }

        cargo_moves.push(line);
    }

    let mut cargo: Vec<VecDeque<char>> = Vec::new();
    for i in (1..(max_cols * 4)).step_by(4) {
        let mut temp_dq: VecDeque<char> = VecDeque::new();
        for c in &cargo_stacks {
            let x = c.chars().nth(i).unwrap();
            if !x.is_whitespace() {
                temp_dq.push_front(x);
            }
        }
        cargo.push(temp_dq);
    }

    // Part 1A
    // for m in cargo_moves {
    //     let move_instr: Vec<&str> = m.split(" ").collect();
    //     let num_to_move = move_instr[1].parse::<usize>().unwrap();
    //     let from = move_instr[3].parse::<usize>().unwrap() - 1;
    //     let to = move_instr[5].parse::<usize>().unwrap() - 1;

    //     // println!("{} {} {}", num_to_move, from, to);

    //     for _ in 0..num_to_move {
    //         if let Some(x) = cargo[from].pop_back() {
    //             cargo[to].push_back(x);
    //         }
    //     }
    // }

    // for mut c in cargo {
    //     print!("{}", c.pop_back().unwrap());
    // }

    // Part 1B
    for m in cargo_moves {
        let move_instr: Vec<&str> = m.split(" ").collect();
        let num_to_move = move_instr[1].parse::<usize>().unwrap();
        let from = move_instr[3].parse::<usize>().unwrap() - 1;
        let to = move_instr[5].parse::<usize>().unwrap() - 1;

        // println!("{} {} {}", num_to_move, from, to);

        let mut temp_dq: VecDeque<char> = VecDeque::new();
        for _ in 0..num_to_move {
            if let Some(x) = cargo[from].pop_back() {
                temp_dq.push_front(x);
            }
        }
        cargo[to].extend(temp_dq);
    }

    for mut c in cargo {
        print!("{}", c.pop_back().unwrap());
    }

    Ok(())
}
