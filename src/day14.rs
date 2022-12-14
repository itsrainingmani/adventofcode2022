use std::{
    error::Error,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
};

use ndarray::Array2;

#[derive(Clone, Debug, PartialEq)]
enum Matter {
    Rock,
    Sand,
    Air,
    Spout,
}

#[derive(Clone, Debug)]
struct Cavern {
    topo: Array2<Matter>,
    spout: (usize, usize),
    row_bounds: (usize, usize), // (min, max)
    col_bounds: (usize, usize), // (min, max)
    sand_count: u16,
}

impl Cavern {
    fn new(input: Vec<String>) -> Cavern {
        let mut cavern: Array2<Matter> = Array2::from_elem((1000, 10000), Matter::Air);
        let start = (0, 500);
        let mut row_bounds: (usize, usize) = (0, 0);
        let mut col_bounds: (usize, usize) = (500, 500);

        cavern[start] = Matter::Spout;

        for line in input {
            let points = line.split(" -> ").collect::<Vec<&str>>();
            for pts in points.as_slice().windows(2) {
                let coords1 = pts[0]
                    .split(",")
                    .map(|p| p.parse::<u16>().unwrap())
                    .collect::<Vec<u16>>();
                let coords2 = pts[1]
                    .split(",")
                    .map(|p| p.parse::<u16>().unwrap())
                    .collect::<Vec<u16>>();

                if coords1[0] == coords2[0] {
                    for x in coords1[1].min(coords2[1])..=coords1[1].max(coords2[1]) {
                        if (x as usize) < row_bounds.0 {
                            row_bounds.0 = x as usize;
                        }
                        if (x as usize) > row_bounds.1 {
                            row_bounds.1 = x as usize;
                        }
                        if (coords1[0] as usize) < col_bounds.0 {
                            col_bounds.0 = coords1[0] as usize;
                        }
                        if (coords1[0] as usize) > col_bounds.1 {
                            col_bounds.1 = coords1[0] as usize;
                        }
                        cavern[(x as usize, coords1[0] as usize)] = Matter::Rock;
                    }
                } else if coords1[1] == coords2[1] {
                    for y in coords1[0].min(coords2[0])..=coords1[0].max(coords2[0]) {
                        if (coords1[1] as usize) < row_bounds.0 {
                            row_bounds.0 = coords1[1] as usize;
                        }
                        if (coords1[1] as usize) > row_bounds.1 {
                            row_bounds.1 = coords1[1] as usize;
                        }
                        if (y as usize) < col_bounds.0 {
                            col_bounds.0 = y as usize;
                        }
                        if (y as usize) > col_bounds.1 {
                            col_bounds.1 = y as usize;
                        }
                        cavern[(coords1[1] as usize, y as usize)] = Matter::Rock;
                    }
                }
            }
        }

        Cavern {
            topo: cavern,
            spout: start,
            row_bounds,
            col_bounds,
            sand_count: 0,
        }
    }

    fn generate_bottom(&mut self) {
        let new_bottom = self.row_bounds.1 + 2;
        self.row_bounds.1 = new_bottom;
        for c in 0..1000 {
            self.topo[(new_bottom, c)] = Matter::Rock;
        }
    }

    // fn count_sand(&self) -> u16 {
    //     let mut num_sand = 0;
    //     for x in self.row_bounds.0..=self.row_bounds.1 {
    //         for y in self.col_bounds.0..=self.col_bounds.1 {
    //             let i = self.topo[(x, y)].clone();
    //             match i {
    //                 Matter::Sand => num_sand += 1,
    //                 _ => {}
    //             };
    //         }
    //     }

    //     num_sand
    // }

    fn possible_sand_moves(&self, sand_pos: (usize, usize)) -> Option<(usize, usize)> {
        let mut next_sand_pos = sand_pos;
        let mut moves = vec![
            (sand_pos.0 + 1, sand_pos.1),
            (sand_pos.0 + 1, sand_pos.1 - 1),
            (sand_pos.0 + 1, sand_pos.1 + 1),
        ];
        moves.retain(|(x, y)| {
            *x < 1000 && *y > 0 && *y < 10000 && self.topo[(*x, *y)] == Matter::Air
        });
        // println!("{:?}", moves);

        if moves.len() == 0 {
            None
        } else {
            Some(*moves.first().unwrap())
        }
    }

    fn is_out_of_bounds(&self, s: (usize, usize)) -> bool {
        s.0 >= self.row_bounds.1 || s.1 < self.col_bounds.0 || s.1 > self.col_bounds.1
    }

    fn sand_production(&mut self) -> (usize, usize) {
        let mut temp_sand_pos = self.spout;

        while let Some(p) = self.possible_sand_moves(temp_sand_pos) {
            temp_sand_pos = p;
        }

        self.topo[temp_sand_pos] = Matter::Sand;
        self.sand_count += 1;
        temp_sand_pos
    }
}

impl fmt::Display for Cavern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in self.row_bounds.0..=self.row_bounds.1 {
            for y in self.col_bounds.0..=self.col_bounds.1 {
                let i = self.topo[(x, y)].clone();
                match i {
                    Matter::Air => write!(f, ".")?,
                    Matter::Rock => write!(f, "#")?,
                    Matter::Sand => write!(f, "o")?,
                    Matter::Spout => write!(f, "+")?,
                };
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn process_part1() -> Result<(), Box<dyn Error>> {
    let file = File::open("inputs/day14.txt")?;
    let reader = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();

    let mut cavern = Cavern::new(reader);

    let mut next_pos = cavern.spout;
    let updated_pos = cavern.sand_production();

    for _ in 0..100_000 {
        next_pos = cavern.sand_production();

        if cavern.is_out_of_bounds(next_pos) {
            break;
        }
    }

    // println!("{}", cavern);
    println!("Part 1 - {}", cavern.sand_count - 1);

    Ok(())
}

fn process_part2() -> Result<(), Box<dyn Error>> {
    let file = File::open("inputs/day14.txt")?;
    let reader = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();

    let mut cavern = Cavern::new(reader);
    cavern.generate_bottom();

    let mut next_pos = cavern.spout;
    cavern.sand_production();

    for _ in 0..100_000 {
        next_pos = cavern.sand_production();
        if next_pos == (0, 500) {
            break;
        }
    }

    // println!("{}", cavern);
    println!("Part 2 - {}", cavern.sand_count);
    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    process_part1()?;
    // process_part2()?;
    Ok(())
}
