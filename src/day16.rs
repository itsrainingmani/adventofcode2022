use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fs::{read_to_string, File},
    io::BufRead,
    io::BufReader,
};

use regex::{Captures, Regex};

#[derive(Debug)]
struct Valve {
    name: String,
    leads_to: Vec<String>,
    flow_rate: u8,
}

fn process_valve(caps: &Captures, line: &str) -> Valve {
    let valve_name = caps.name("name").unwrap();
    let flow_rate = caps.name("rate").unwrap();
    let valves = caps.name("valves").unwrap();

    let (a, ae) = (valve_name.start(), valve_name.end());
    let (bs, be) = (flow_rate.start(), flow_rate.end());
    let (cs, ce) = (valves.start(), valves.end());
    Valve {
        name: line[a..ae].to_string(),
        leads_to: line[cs..ce]
            .to_string()
            .split(", ")
            .map(|s| s.to_owned())
            .collect::<Vec<String>>(),
        flow_rate: line[bs..be].parse::<u8>().unwrap(),
    }
}

fn process_part1() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("inputs-test/day16.txt")?;
    let valve_re: Regex = Regex::new(
        r"Valve (?P<name>\w+) has flow rate=(?P<rate>\d+); tunnels lead to valves (?P<valves>.+)",
    )?;
    let mut valves: Vec<Valve> = vec![];
    for line in input.lines() {
        println!("{}", line);
        if valve_re.is_match(line) {
            let caps = valve_re.captures(line).unwrap();
            valves.push(process_valve(&caps, line));
        }
    }
    println!("{:?}", valves);
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
