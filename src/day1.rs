use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{collections::VecDeque, error::Error};

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("\nDay 1 - Maximum Elf Calories");

    // We are only trying to find the highest calories being held by any elf
    // To do this, we will parse each line. When we see a new line we will check
    // if the current number of calories is the new max

    // Day 1A - 67622
    println!("\tPart 1A - {}", find_max_calories().unwrap());

    // Day 1B
    println!("\tPart 1B - {}", find_top3_calorie_sum().unwrap());

    Ok(())
}

fn find_max_calories() -> Result<u64, Box<dyn Error>> {
    let file = File::open("inputs/day1.txt")?;
    let reader = BufReader::new(file);

    let mut current_max: u64 = u64::MIN;
    let mut cals_of_current_elf: u64 = u64::MIN;

    for line in reader.lines().map(|l| l.unwrap()) {
        if line.len() > 0 {
            // extract the numerical caloric value
            cals_of_current_elf += line.parse::<u64>()?;
        } else {
            // check if current cal count is more than max
            if cals_of_current_elf > current_max {
                current_max = cals_of_current_elf;
            }

            cals_of_current_elf = 0;
        }
    }

    Ok(current_max)
}

fn find_top3_calorie_sum() -> Result<u64, Box<dyn Error>> {
    let file = File::open("inputs/day1.txt")?;
    let reader = BufReader::new(file);

    let mut top_3_max: VecDeque<u64> = VecDeque::from([0, 0, 0]);
    let mut cals_of_current_elf: u64 = u64::MIN;

    for line in reader.lines().map(|l| l.unwrap()) {
        if line.len() > 0 {
            // extract the numerical caloric value
            cals_of_current_elf += line.parse::<u64>()?;
        } else {
            // check if current cal count is more than max

            if cals_of_current_elf > *top_3_max.get(0).unwrap() {
                top_3_max.pop_back();
                top_3_max.push_front(cals_of_current_elf);
            } else if cals_of_current_elf > *top_3_max.get(1).unwrap() {
                top_3_max.insert(1, cals_of_current_elf);
                top_3_max.pop_back();
            } else if cals_of_current_elf > *top_3_max.get(2).unwrap() {
                top_3_max.pop_back();
                top_3_max.push_back(cals_of_current_elf);
            }

            cals_of_current_elf = 0;
        }
    }

    Ok(top_3_max.iter().fold(0u64, |sum, i| sum + (*i as u64)))
}
