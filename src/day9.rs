use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_taxicab_dist1() {
        let head = Pos { row: 1, col: 1 };
        let tail = Pos { row: 0, col: 0 };

        assert_eq!(taxicab_dist(head, tail), 2);
    }

    #[test]
    fn check_taxicab_dist2() {
        let head = Pos { row: 2, col: 1 };
        let tail = Pos { row: 0, col: 0 };

        assert_eq!(taxicab_dist(head, tail), 3);
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Pos {
    row: i16,
    col: i16,
}

fn taxicab_dist(ahead: Pos, behind: Pos) -> u16 {
    ahead.row.abs_diff(behind.row) + ahead.col.abs_diff(behind.col)
}

fn is_adjacent(ahead: Pos, behind: Pos) -> bool {
    if ahead.col != behind.col && ahead.row != behind.row {
        return taxicab_dist(ahead, behind) <= 2;
    } else {
        if ahead.row == behind.row && ahead.col != behind.col {
            return taxicab_dist(ahead, behind) <= 1;
        } else if ahead.row != behind.row && ahead.col == behind.col {
            return taxicab_dist(ahead, behind) <= 1;
        } else {
            true
        }
    }
}

fn simulate_rope(num_knots: usize) -> Result<(), Box<dyn Error>> {
    let file = File::open("../inputs/day9.txt")?;
    let reader = BufReader::new(file);

    let mut visited_positions: HashSet<Pos> = HashSet::new();

    let mut rope: Vec<Pos> = Vec::new();
    for _ in 0..num_knots {
        rope.push(Pos { row: 0, col: 0 });
    }

    for line in reader.lines().map(|l| l.unwrap()) {
        let instructions: Vec<&str> = line.split(" ").collect();
        let direction = instructions[0];
        let number_of_moves = instructions[1].parse::<i16>().unwrap();

        for _ in 0..number_of_moves {
            let mut head_knot = rope.get_mut(0).unwrap();
            match direction {
                "R" => head_knot.col += 1,
                "L" => head_knot.col -= 1,
                "U" => head_knot.row += 1,
                "D" => head_knot.row -= 1,
                _ => {}
            }
            for i in 1..num_knots {
                let head = rope[i - 1];
                let mut tail = rope.get_mut(i).unwrap();

                if !is_adjacent(head, *tail) {
                    let new_position = where_to_move(head, *tail);
                    tail.row = new_position.row;
                    tail.col = new_position.col;
                }
            }
            // println!("{:?}", rope);
            visited_positions.insert(*rope.last().unwrap());
        }
    }
    // println!("{:?}", rope.last().unwrap());
    println!("{}", visited_positions.len());

    Ok(())
}

fn where_to_move(ahead: Pos, behind: Pos) -> Pos {
    let mut new_pos = behind.clone();
    // handle the cardinal moves first
    if ahead.row == new_pos.row && ahead.col != new_pos.col {
        if ahead.col > new_pos.col {
            // Move to the right
            new_pos.col += 1;
        } else if ahead.col < new_pos.col {
            // Move to the left
            new_pos.col -= 1;
        }
        return new_pos;
    } else if ahead.col == new_pos.col && ahead.row != new_pos.row {
        if ahead.row > new_pos.row {
            // Move up
            new_pos.row += 1;
        } else if ahead.row < new_pos.row {
            // Move down
            new_pos.row -= 1;
        }
        return new_pos;
    } else if ahead.row != new_pos.row && ahead.col != new_pos.col {
        if ahead.row.abs_diff(new_pos.row) == 2 && ahead.col.abs_diff(new_pos.col) < 2 {
            // we are moving to the same column regardless
            new_pos.col = ahead.col;
            if ahead.row > new_pos.row {
                new_pos.row += 1;
            } else {
                new_pos.row -= 1;
            }
            return new_pos;
        } else if ahead.col.abs_diff(new_pos.col) == 2 && ahead.row.abs_diff(new_pos.row) < 2 {
            // we are moving to the same row regardless
            new_pos.row = ahead.row;
            if ahead.col > new_pos.col {
                new_pos.col += 1;
            } else {
                new_pos.col -= 1;
            }
            return new_pos;
        } else if ahead.col.abs_diff(new_pos.col) == 2 && ahead.row.abs_diff(new_pos.row) == 2 {
            if ahead.row > behind.row {
                if ahead.col > behind.col {
                    new_pos.row += 1;
                    new_pos.col += 1;
                } else {
                    new_pos.row += 1;
                    new_pos.col -= 1;
                }
            } else {
                if ahead.col > behind.col {
                    new_pos.row -= 1;
                    new_pos.col += 1;
                } else {
                    new_pos.row -= 1;
                    new_pos.col -= 1;
                }
            }
        }
    }

    new_pos
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("\nDay 9");

    simulate_rope(2)?;
    simulate_rope(10)?;

    Ok(())
}
