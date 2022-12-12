extern crate regex;

use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    fs::read_to_string,
    ops::Rem,
};

use regex::{Match, Regex};

//           items     op       val     by  true   false
#[derive(Debug)]
struct Monkey(VecDeque<u64>, String, String, u64, usize, usize);

#[derive(Debug)]
struct NoWorryMonkey(VecDeque<Arith>, String, String, u64, usize, usize);

#[derive(Debug, Clone)]
struct Arith {
    ops: Vec<String>,
}

impl Arith {
    fn parse(&self, div_by: u64) -> bool {
        true
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let monkey_re = Regex::new(r"Monkey (?P<index>[0-9]+)").unwrap();
    let items_re = Regex::new(r"Starting items: (?P<items>.+)").unwrap();
    let op_re = Regex::new(r"Operation: new = old (?P<op>.) (?P<val>.*)").unwrap();
    let test_re = Regex::new(r"Test: divisible by (?P<div_by>[0-9]+)").unwrap();
    let true_re = Regex::new(r"If true: throw to monkey (?P<index>[0-9]+)").unwrap();
    let false_re = Regex::new(r"If false: throw to monkey (?P<index>[0-9]+)").unwrap();

    let input = read_to_string("inputs/day11.txt")?;
    let num_monkeys = input.matches("Monkey").collect::<Vec<&str>>().len();
    let mut monkeys: Vec<Monkey> = vec![];
    let mut monkey_count: Vec<u64> = vec![];
    let mut item_stack: HashMap<usize, VecDeque<u64>> = HashMap::new();
    for i in 0..num_monkeys {
        monkeys.push(Monkey(
            VecDeque::new(),
            String::new(),
            String::new(),
            0,
            0,
            0,
        ));
        monkey_count.push(0);
        item_stack.insert(i, VecDeque::new());
    }
    let mut cur_idx: usize = 0;
    let mut caps: Match;
    let (mut start, mut end) = (0, 0);
    let mut monke = monkeys.get_mut(cur_idx).unwrap();

    for line in input.split("\n") {
        if monkey_re.is_match(line) {
            caps = monkey_re.captures(line).unwrap().name("index").unwrap();
            (start, end) = (caps.start(), caps.end());
            cur_idx = line[start..end].parse::<usize>().unwrap();
            // println!("{:?}", cur_idx);
            monke = monkeys.get_mut(cur_idx).unwrap();
        } else if items_re.is_match(line) {
            caps = items_re.captures(line).unwrap().name("items").unwrap();
            (start, end) = (caps.start(), caps.end());
            let items = line[start..end]
                .split(", ")
                .map(|i| i.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            monke.0 = VecDeque::from(items);
        } else if op_re.is_match(line) {
            caps = op_re.captures(line).unwrap().name("op").unwrap();
            (start, end) = (caps.start(), caps.end());
            let op = line[start..end].to_string();
            caps = op_re.captures(line).unwrap().name("val").unwrap();
            (start, end) = (caps.start(), caps.end());
            let val = line[start..end].to_string();
            // println!("{:?} {:?}", op, val);
            monke.1 = op;
            monke.2 = val;
        } else if test_re.is_match(line) {
            caps = test_re.captures(line).unwrap().name("div_by").unwrap();
            (start, end) = (caps.start(), caps.end());
            let div_by = line[start..end].parse::<u64>().unwrap();
            // println!("{:?}", div_by);
            monke.3 = div_by;
        } else if true_re.is_match(line) {
            caps = true_re.captures(line).unwrap().name("index").unwrap();
            (start, end) = (caps.start(), caps.end());
            let true_index = line[start..end].parse::<usize>().unwrap();
            // println!("{:?}", true_index);
            monke.4 = true_index;
        } else if false_re.is_match(line) {
            caps = false_re.captures(line).unwrap().name("index").unwrap();
            (start, end) = (caps.start(), caps.end());
            let false_index = line[start..end].parse::<usize>().unwrap();
            // println!("{:?}", false_index);
            monke.5 = false_index;
        }
    }

    for _ in 0..20 {
        for (cur_idx, monk) in monkeys.iter_mut().enumerate() {
            let cur_items = item_stack.get_mut(&cur_idx).unwrap();
            if cur_items.len() > 0 {
                monk.0.extend(cur_items.iter());
                cur_items.clear();
            }
            while !monk.0.is_empty() {
                let item = monk.0.pop_front().unwrap();
                monkey_count[cur_idx] += 1;
                let mut item_worry;
                if monk.2 == "old" {
                    if monk.1 == "*" {
                        item_worry = item * item;
                    } else {
                        item_worry = item + item;
                    }
                } else {
                    let val = monk.2.parse::<u64>().unwrap();
                    if monk.1 == "*" {
                        item_worry = item * val;
                    } else {
                        item_worry = item + val;
                    }
                }
                // divide by 3 to the nearest integer
                item_worry = (item_worry as f32 / 3.0) as u64;
                if item_worry.rem(monk.3) == 0 {
                    item_stack.get_mut(&monk.4).unwrap().push_back(item_worry);
                } else {
                    item_stack.get_mut(&monk.5).unwrap().push_back(item_worry);
                }
            }
        }
    }
    monkey_count.sort_by(|a, b| b.cmp(&a));
    let top_2 = &monkey_count[0..=1].iter().fold(1, |acc, x| acc * x);
    println!("Part 1 - {:?}", top_2);

    let mut no_worry: Vec<NoWorryMonkey> = vec![];
    monkey_count.clear();
    item_stack.clear();

    for i in 0..num_monkeys {
        no_worry.push(NoWorryMonkey(
            VecDeque::new(),
            String::new(),
            String::new(),
            0,
            0,
            0,
        ));
        monkey_count.push(0);
        item_stack.insert(i, VecDeque::new());
    }

    for _ in 0..10_000 {
        for (cur_idx, monk) in monkeys.iter_mut().enumerate() {
            let cur_items = item_stack.get_mut(&cur_idx).unwrap();
            if cur_items.len() > 0 {
                monk.0.extend(cur_items.iter());
                cur_items.clear();
            }
            while !monk.0.is_empty() {
                let item = monk.0.pop_front().unwrap();
                monkey_count[cur_idx] += 1;
                let item_worry;
                let val = if monk.2 == "old" {
                    item
                } else {
                    monk.2.parse::<u64>().unwrap()
                };
                if monk.1 == "*" {
                    item_worry = item * val;
                } else {
                    item_worry = item + val;
                }
                // item_worry = (item_worry as f32 / 3.0) as u64;
                if item_worry.rem(monk.3) == 0 {
                    item_stack.get_mut(&monk.4).unwrap().push_back(item_worry);
                } else {
                    item_stack.get_mut(&monk.5).unwrap().push_back(item_worry);
                }
            }
        }
    }

    Ok(())
}
