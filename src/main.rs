use std::error::Error;

use adventofcode2022::{day1, day2, day3, day4, day5, day6};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Advent of Code 2022");
    day1::main()?;
    day2::main()?;
    day3::main()?;
    day4::main()?;
    day5::main()?;
    day6::main();

    Ok(())
}
