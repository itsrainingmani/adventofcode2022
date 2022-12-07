use std::{error::Error, fs::File, io::BufRead, io::BufReader};

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("\nDay 4");

    let file = File::open("inputs/day4.txt")?;
    let reader = BufReader::new(file);

    let mut ranges_subsumed: u32 = 0;
    let mut ranges_overlapped: u32 = 0;

    for line in reader.lines().map(|l| l.unwrap()) {
        let section_assignment: Vec<&str> = line.split(',').collect();

        let sec1: Vec<_> = section_assignment[0]
            .split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let sec2: Vec<_> = section_assignment[1]
            .split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        if ((sec1[0] <= sec2[0]) && (sec1[1] >= sec2[1]))
            || ((sec2[0] <= sec1[0]) && (sec2[1] >= sec1[1]))
        {
            ranges_subsumed += 1;
        }

        if !(sec1[0] > sec2[1] || sec1[1] < sec2[0]) {
            ranges_overlapped += 1;
        }
    }

    println!("\tPart 4A - Ranges Subsumed - {}", ranges_subsumed);
    println!("\tPart 4B - Ranges Overlapped - {}", ranges_overlapped);

    Ok(())
}
