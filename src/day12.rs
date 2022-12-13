mod tests {
    use super::*;
    #[test]
    fn check_bounds() {
        let input = read_to_string("inputs/day12-test.txt").unwrap();
        let mut hill_map = HeightMap::new(input);
        let visited: Array2<u8> = Array2::zeros((5, 8));

        assert_eq!(hill_map.get_adj((0, 0)), Some(vec![(0, 1), (1, 0)]));
        // assert_eq!(hill_map.get_adj((0, 4)), Some(vec![(0, 5), (0, 3)]));
        assert_eq!(
            hill_map.get_adj((2, 4)),
            Some(vec![(2, 5), (2, 3), (3, 4), (1, 4)])
        );
    }
}

extern crate ndarray;

use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fs::read_to_string,
};

use ndarray::Array2;

#[derive(Debug)]
struct HeightMap {
    width: usize,
    height: usize,
    start: (usize, usize),
    grid: Array2<char>,
}

impl HeightMap {
    fn new(lines: String) -> HeightMap {
        let mut tmp_grd: Vec<Vec<char>> = Vec::new();

        let mut start = (0, 0);

        for (x, l) in lines.split("\n").enumerate() {
            let mut tmp_row: Vec<char> = Vec::new();
            for (y, c) in l.chars().enumerate() {
                if c == 'S' {
                    start = (x, y);
                }
                tmp_row.push(c);
            }
            tmp_grd.push(tmp_row);
        }
        let height = tmp_grd.len();
        let width = tmp_grd[0].len();

        let mut grid = Array2::from_shape_vec(
            (height, width),
            tmp_grd.iter().flatten().map(|c| *c).collect::<Vec<char>>(),
        )
        .unwrap();

        HeightMap {
            width,
            height,
            start,
            grid,
        }
    }

    fn find_positions_for_height(&self, to_find: char) -> Option<Vec<(usize, usize)>> {
        let mut found_positions = Vec::<(usize, usize)>::new();

        for (pos, c) in self.grid.indexed_iter() {
            if to_find == 'S' {
                if *c == to_find || *c == 'a' {
                    found_positions.push(pos);
                }
            } else if *c == to_find {
                found_positions.push(pos);
            }
        }

        if found_positions.len() > 0 {
            Some(found_positions)
        } else {
            None
        }
    }

    // get the adjacent vectors that you can visit
    fn get_adj(&self, (i, j): (usize, usize)) -> Option<Vec<(usize, usize)>> {
        let height_at_pos = self.grid.get((i, j)).unwrap();
        let mut adj_nodes = Vec::<(usize, usize)>::new();
        let mut positions_to_check = Vec::<(usize, usize)>::new();

        if j == 0 {
            positions_to_check.push((i, j + 1));
            if i == 0 {
                positions_to_check.push((i + 1, j));
            } else if i == self.grid.dim().0 - 1 {
                positions_to_check.push((i - 1, j));
            } else {
                positions_to_check.push((i + 1, j));
                positions_to_check.push((i - 1, j));
            }
        } else if j == (self.grid.dim().1 - 1) {
            positions_to_check.push((i, j - 1));
            if i == 0 {
                positions_to_check.push((i + 1, j));
            } else if i == self.grid.dim().0 - 1 {
                positions_to_check.push((i - 1, j));
            } else {
                positions_to_check.push((i + 1, j));
                positions_to_check.push((i - 1, j));
            }
        } else {
            positions_to_check.push((i, j + 1));
            positions_to_check.push((i, j - 1));
            if i == 0 {
                positions_to_check.push((i + 1, j));
            } else if i == self.grid.dim().0 - 1 {
                positions_to_check.push((i - 1, j));
            } else {
                positions_to_check.push((i + 1, j));
                positions_to_check.push((i - 1, j));
            }
        }

        if positions_to_check.len() > 0 {
            for pos in positions_to_check {
                let cur_height = self.grid.get(pos).unwrap();
                if *height_at_pos == 'S' {
                    if *cur_height == 'a' || *cur_height == 'b' {
                        adj_nodes.push(pos);
                    }
                } else {
                    if *cur_height == 'E' {
                        if *height_at_pos == 'y' || *height_at_pos == 'z' {
                            adj_nodes.push(pos);
                        }
                    } else if *cur_height as i8 - *height_at_pos as i8 <= 1 {
                        adj_nodes.push(pos);
                    }
                }
            }
        }

        if adj_nodes.len() > 0 {
            Some(adj_nodes)
        } else {
            None
        }
    }

    fn get_shortest_path(&self, starting: (usize, usize)) -> u32 {
        let grid_shape = self.grid.shape();
        let mut dist: Array2<u32> = Array2::from_elem((grid_shape[0], grid_shape[1]), u32::MAX);
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut queue = VecDeque::<(usize, usize)>::new();

        queue.push_back(starting);
        visited.insert(starting);

        *dist.get_mut(starting).unwrap() = 0;

        while !queue.is_empty() {
            // println!("visited - {:?}", visited);
            // println!("queue - {:?}", queue);
            let s = queue.pop_front().unwrap();
            let height = *self.grid.get(s).unwrap();
            if height == 'E' {
                return *dist.get(s).unwrap();
            }
            if let Some(adj_nodes) = self.get_adj(s) {
                for a in adj_nodes {
                    if !visited.contains(&a) {
                        *dist.get_mut(a).unwrap() = dist.get(s).unwrap() + 1;
                        queue.push_back(a);
                        visited.insert(a);
                    }
                }
            } else {
                continue;
            }
        }

        0
    }
}

impl std::fmt::Display for HeightMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Start\n")?;
        for i in self.grid.outer_iter() {
            for j in i.iter() {
                write!(f, "| {} |", j)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "End")
    }
}

fn process_part1() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("inputs/day12.txt")?;
    let hill_map: HeightMap = HeightMap::new(input);
    // println!("{}", hill_map);
    println!(
        "Fewess steps - {}",
        hill_map.get_shortest_path(hill_map.start)
    );

    Ok(())
}

fn process_part2() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("inputs/day12.txt")?;
    let hill_map = HeightMap::new(input);
    let mut fewest_steps = Vec::<u32>::new();
    // println!("{}", hill_map);

    if let Some(lowest_elevs) = hill_map.find_positions_for_height('a') {
        for elev in lowest_elevs {
            let shortest_path = hill_map.get_shortest_path(elev);
            fewest_steps.push(shortest_path);
        }
        fewest_steps = fewest_steps
            .iter()
            .filter(|&f| *f != 0)
            .map(|f| *f)
            .collect();
        fewest_steps.sort();
        println!(
            "Fewest steps from any elev a - {}",
            fewest_steps.first().unwrap()
        );
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    process_part1()?;
    process_part2()?;

    Ok(())
}
