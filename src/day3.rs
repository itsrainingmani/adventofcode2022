use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("\nDay 3");

    let file = File::open("inputs/day3.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    calculate_rucksack_item_prio(lines.clone());
    calculate_elf_badges(lines);

    Ok(())
}

fn priority(c: char) -> u32 {
    c as u32 - if c.is_ascii_lowercase() { 96 } else { 38 }
}

fn calculate_elf_badges(lines: Vec<String>) {
    let mut item_priority_sum: u32 = 0;
    let elf_triumvirates = lines.chunks(3);

    for elves in elf_triumvirates {
        let elf1_items: HashSet<char> = HashSet::from_iter(elves[0].chars());
        let elf2_items: HashSet<char> = HashSet::from_iter(elves[1].chars());
        let elf3_items: HashSet<char> = HashSet::from_iter(elves[2].chars());

        let common_item: Vec<char> = elf1_items
            .intersection(&elf2_items.intersection(&elf3_items).map(|c| *c).collect())
            .map(|c| *c)
            .collect();

        item_priority_sum += priority(*common_item.get(0).unwrap());
    }

    println!("\t Part 1A - {}", item_priority_sum);
}

fn calculate_rucksack_item_prio(lines: Vec<String>) {
    let mut item_priority_sum: u32 = 0;
    let mut rucksack_items: HashSet<char> = HashSet::new();

    for line in lines {
        let line_len = line.len();
        let compartment1: HashSet<char> = HashSet::from_iter(line[..(line_len / 2)].chars());
        let compartment2: HashSet<char> = HashSet::from_iter(line[(line_len / 2)..].chars());

        let common_item: Vec<char> = compartment1
            .intersection(&compartment2)
            .map(|c| *c)
            .collect();
        item_priority_sum += priority(*common_item.get(0).unwrap());

        rucksack_items.clear();
    }

    println!("\t Part 1B - {}", item_priority_sum);
}
