use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fs::{read_to_string, File},
    io::BufRead,
    io::BufReader,
};

#[derive(Debug, Clone, Copy)]
enum RockType {
    Hori,
    Verti,
    Square,
    L,
    Cross,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Down,
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone)]
struct Rock {
    kind: RockType,
    size: usize,
    indices: Vec<Pos>, // since not all rocks have 5 indices, the last one will be -1 if it doesn't exist
}

impl Rock {
    fn generate_rock(floor: u32, kind: RockType) -> Rock {
        let mut indices = vec![];
        let mut size = 4;
        let rb = floor + 4; // rock bottom

        match kind {
            RockType::Hori => {
                indices = vec![
                    Pos { x: rb, y: 2 },
                    Pos { x: rb, y: 3 },
                    Pos { x: rb, y: 4 },
                    Pos { x: rb, y: 5 },
                ];
                size = 4;
            }
            RockType::Verti => {
                indices = vec![
                    Pos { x: rb + 3, y: 2 },
                    Pos { x: rb + 2, y: 2 },
                    Pos { x: rb + 1, y: 2 },
                    Pos { x: rb, y: 2 },
                ];
                size = 4;
            }
            RockType::Square => {
                indices = vec![
                    Pos { x: rb, y: 2 },
                    Pos { x: rb + 1, y: 2 },
                    Pos { x: rb, y: 3 },
                    Pos { x: rb + 1, y: 3 },
                ];
                size = 4;
            }
            RockType::L => {
                indices = vec![
                    Pos { x: rb, y: 2 },
                    Pos { x: rb, y: 3 },
                    Pos { x: rb, y: 4 },
                    Pos { x: rb + 1, y: 4 },
                    Pos { x: rb + 2, y: 4 },
                ];
                size = 5;
            }
            RockType::Cross => {
                indices = vec![
                    Pos { x: rb, y: 3 },
                    Pos { x: rb + 1, y: 2 },
                    Pos { x: rb + 1, y: 3 },
                    Pos { x: rb + 1, y: 4 },
                    Pos { x: rb + 2, y: 3 },
                ];
                size = 5;
            }
        }
        Rock {
            size,
            kind,
            indices,
        }
    }

    fn left_most(&self) -> &Pos {
        self.indices
            .iter()
            .min_by(|&p1, &p2| p1.y.cmp(&p2.y))
            .unwrap()
    }

    fn right_most(&self) -> &Pos {
        self.indices
            .iter()
            .max_by(|&p1, &p2| p1.y.cmp(&p2.y))
            .unwrap()
    }

    fn bottom_most(&self) -> &Pos {
        self.indices
            .iter()
            .min_by(|&p1, &p2| p1.x.cmp(&p2.x))
            .unwrap()
    }

    fn top_most(&self) -> &Pos {
        self.indices
            .iter()
            .max_by(|&p1, &p2| p1.x.cmp(&p2.x))
            .unwrap()
    }

    // If it's possible to update the rock position
    fn left_or_right(&mut self, dir: char, cavern: &mut VecDeque<[bool; 7]>) {
        let mut indices_to_check: Vec<Pos> = vec![];

        match self.kind {
            RockType::Cross => {
                let indices_to_check: Vec<Pos> = vec![self.indices[]];
            }
            RockType::Hori => todo!(),
            RockType::Verti => todo!(),
            RockType::Square => todo!(),
            RockType::L => todo!(),
        }
    }

    fn move_down(&mut self, cavern: &mut VecDeque<[bool; 7]>) -> Option<u32> {
        todo!()
    }
}

fn process_part1() -> Result<(), Box<dyn Error>> {
    // jets
    // let jets = read_to_string("inputs/day17.txt")?
    // .chars()
    // .collect::<Vec<char>>();
    let input = read_to_string("inputs-test/day17.txt")?;
    let mut jets = input.chars().cycle();
    // Continuosly generate rocks
    let mut rock_iter = [
        RockType::Hori,
        RockType::Cross,
        RockType::L,
        RockType::Verti,
        RockType::Square,
    ]
    .iter()
    .cycle();
    let mut cavern: VecDeque<[bool; 7]> = VecDeque::new(); // update the cavern whenever a given rock has fallen
    let mut top_most: u32 = 0; // This will keep getting updated
    let mut num_fallen_rocks = 0;

    while num_fallen_rocks < 2023 {
        // get the rock type to generate
        let new_rock = Rock::generate_rock(top_most, *rock_iter.next().unwrap());

        let mut fallen = false;
        while !fallen {
            // get the jet direction to move the rock in
            let jet_direction = jets.next().unwrap();

            // check if the rock is able to be moved in the jet direction

            // check if rock was able to fall down. If not, update the fallen flag.
            // increment the number of fallen rocks
        }
    }

    // calculate the longest unbroken chain of columns

    Ok(())
}

fn process_part2() -> Result<(), Box<dyn Error>> {
    todo!();
    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    process_part1()?;
    // process_part2()?;
    Ok(())
}

mod test {
    use super::*;

    #[test]
    fn test_rock_generation() {
        let rock = Rock::generate_rock(3, RockType::Cross);
        println!("{:?}", Rock::generate_rock(3, RockType::Cross));
        println!("{:?}", Rock::generate_rock(3, RockType::Hori));
        println!("{:?}", Rock::generate_rock(3, RockType::Verti));
        println!("{:?}", Rock::generate_rock(3, RockType::L));
        println!("{:?}", Rock::generate_rock(3, RockType::Square));

        println!(
            "{:?} {:?} {:?} {:?}",
            rock.left_most(),
            rock.right_most(),
            rock.top_most(),
            rock.bottom_most()
        );
    }
}
