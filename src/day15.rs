use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::read_to_string,
};

use regex::{Captures, Regex};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Border {
    line: Vec<Pos>,
}

fn sensor_indices(caps: &Captures, line: &str) -> Pos {
    let col = caps.name("col").unwrap();
    let row = caps.name("row").unwrap();
    let (cs, ce) = (col.start(), col.end());
    let (bs, be) = (row.start(), row.end());
    Pos {
        x: line[bs..be].parse::<i32>().unwrap(),
        y: line[cs..ce].parse::<i32>().unwrap(),
    }
}

fn beacon_indices(caps: &Captures, line: &str) -> Pos {
    let col = caps.name("bcol").unwrap();
    let row = caps.name("brow").unwrap();
    let (cs, ce) = (col.start(), col.end());
    let (bs, be) = (row.start(), row.end());
    Pos {
        x: line[bs..be].parse::<i32>().unwrap(),
        y: line[cs..ce].parse::<i32>().unwrap(),
    }
}

fn manhattan_distance(p1: Pos, p2: Pos) -> i32 {
    p1.x.abs_diff(p2.x) as i32 + p1.y.abs_diff(p2.y) as i32
}

fn reverse_manhattan(sensor: Pos, beacon: Pos, row: i32) -> (i32, i32) {
    let mh_dist = manhattan_distance(sensor, beacon);
    let top_row = sensor.x - mh_dist;
    let bottom_row = sensor.x + mh_dist;
    let mut left_intersect = sensor.y;
    let mut right_intersect = sensor.y;
    let mut diff = 0;

    if row > sensor.x {
        diff = bottom_row - row;
    } else if row < sensor.x {
        diff = row - top_row;
    }

    left_intersect -= diff;
    right_intersect += diff;

    (left_intersect, right_intersect)
}

fn print_grid(sense_map: HashMap<Pos, Pos>) {
    let mut grid = [["."; 30]; 30];

    for (&sensor, &beacon) in sense_map.iter() {
        let radius = manhattan_distance(sensor, beacon);
        if sensor.x >= 0 && sensor.x < 30 && sensor.y >= 0 && sensor.y < 30 {
            let s_idx = (sensor.x as usize, sensor.y as usize);
            grid[s_idx.0][s_idx.1] = "S";
        }
        if beacon.x >= 0 && beacon.x < 30 && beacon.y >= 0 && beacon.y < 30 {
            let b_idx = (beacon.x as usize, beacon.y as usize);
            grid[b_idx.0][b_idx.1] = "B";
        }

        for rows in grid {
            println!("{}", rows.join(""));
        }
    }
}

fn perimeter(sensor: Pos, beacon: Pos, max_val: i32) -> Border {
    // let mut top_right = top_right_border(sensor, beacon, max_val);
    let radius = manhattan_distance(sensor, beacon);
    let mut top_right = ((sensor.x - radius - 1)..=(sensor.x))
        .zip((sensor.y)..=(sensor.y + radius + 1))
        .collect::<Vec<(i32, i32)>>();

    top_right
        .extend(((sensor.x - radius - 1)..=(sensor.x)).zip((sensor.y - radius - 1)..=(sensor.y)));
    top_right
        .extend(((sensor.x)..=(sensor.x + radius + 1)).zip((sensor.y)..=(sensor.y + radius + 1)));
    top_right.extend(
        ((sensor.x)..=(sensor.x + radius + 1)).zip((sensor.y - radius as i32 - 1)..=(sensor.y)),
    );
    let top_right = top_right
        .iter()
        .filter(|(x, y)| *x >= 0 && *x <= max_val && *y >= 0 && *y <= max_val)
        .map(|(x, y)| Pos { x: *x, y: *y })
        .collect::<Vec<Pos>>();
    Border { line: top_right }
}

fn process_part1() -> Result<(), Box<dyn Error>> {
    // let input = read_to_string("inputs-test/day15.txt")?;
    // let row_to_match = 10;

    let input = read_to_string("inputs/day15.txt")?;
    let row_to_match = 2_000_000;

    let sensor_re: Regex = Regex::new(r"Sensor at x=(?P<col>\d+), y=(?P<row>\d+): closest beacon is at x=(?P<bcol>-?\d+), y=(?P<brow>-?\d+)").unwrap();

    let mut result = 0;
    let mut beacon_set: HashSet<Pos> = HashSet::new();
    let mut index_map: HashMap<Pos, Vec<Pos>> = HashMap::new();

    for line in input.lines() {
        if sensor_re.is_match(line) {
            let caps = sensor_re.captures(line).unwrap();
            let sensor = sensor_indices(&caps, line);
            let beacon = beacon_indices(&caps, line);

            let mh_dist = manhattan_distance(sensor, beacon);
            if row_to_match <= (sensor.x + mh_dist) && row_to_match >= (sensor.x - mh_dist) {
                // make sure that the vec is initialized before inserting the next value
                index_map.entry(beacon).or_insert(Vec::new()).push(sensor);
                beacon_set.insert(beacon);
            }
        }
    }

    let mut row_intercepts: Vec<(Pos, Pos)> = vec![];

    for beacon in beacon_set.iter() {
        let sensors = &index_map[beacon];
        for sensor in sensors {
            let r_mh_dist = reverse_manhattan(*sensor, *beacon, row_to_match);
            row_intercepts.push((
                Pos {
                    x: row_to_match,
                    y: r_mh_dist.0,
                },
                Pos {
                    x: row_to_match,
                    y: r_mh_dist.1,
                },
            ));
        }
    }

    row_intercepts.sort_by(|a, b| a.0.y.partial_cmp(&b.0.y).unwrap());
    println!("{:?}", row_intercepts);
    let mut farthest_left = i32::MIN;

    for (left, right) in row_intercepts {
        if farthest_left > right.y {
            continue;
        } else if left.y >= farthest_left {
            farthest_left = right.y;
            result += right.y - left.y;
        } else if right.y > farthest_left && left.y <= farthest_left {
            result += right.y - farthest_left;
            farthest_left = right.y;
        }
    }
    println!("{}", result);

    Ok(())
}

fn process_part2() -> Result<(), Box<dyn Error>> {
    // let input = read_to_string("inputs-test/day15.txt")?;
    // let max_val = 20;

    let input = read_to_string("inputs/day15.txt")?;
    let max_val = 4_000_000;

    let sensor_re: Regex = Regex::new(r"Sensor at x=(?P<col>\d+), y=(?P<row>\d+): closest beacon is at x=(?P<bcol>-?\d+), y=(?P<brow>-?\d+)").unwrap();

    let mut result: i64 = 0;
    let mut sense_map: HashMap<Pos, Pos> = HashMap::new();

    for line in input.lines() {
        if sensor_re.is_match(line) {
            let caps = sensor_re.captures(line).unwrap();
            sense_map.insert(sensor_indices(&caps, line), beacon_indices(&caps, line));
        }
    }

    for (&sensor, &beacon) in sense_map.iter() {
        let perimeter = perimeter(sensor, beacon, max_val);
        let mut perimeter_pt = Pos { x: 0, y: 0 };
        for pt in perimeter.line {
            let peri_calc = sense_map
                .iter()
                .map(|(s1, b1)| (manhattan_distance(*s1, pt)) - (manhattan_distance(*s1, *b1)))
                .filter(|x| *x >= 1)
                .collect::<Vec<i32>>();
            if peri_calc.len() == sense_map.len() {
                println!("{:?}", (pt.y as i64) * (max_val as i64) + pt.x as i64);
                perimeter_pt = pt;
                break;
            }
        }
        if perimeter_pt.x != 0 && perimeter_pt.y != 0 {
            break;
        }
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    // process_part1()?;
    process_part2()?;
    Ok(())
}

mod test {
    use super::*;

    #[test]
    fn check_intersects() {
        let sensor = Pos { x: 12, y: 4 };
        let beacon = Pos { x: 8, y: 6 };
        let row_watch = 10;

        let (linter, rinter) = reverse_manhattan(sensor, beacon, row_watch);

        println!("{:?}", (row_watch, linter));
        println!("{:?}", (row_watch, rinter));
    }
}
