use std::{
    cmp::Ordering,
    error::Error,
    fs::{read_to_string, File},
    io::{BufRead, BufReader},
};

fn correctorder(pair: &str) -> bool {
    let (lhs, rhs) = pair.split_once('\n').unwrap();
    compare(lhs.as_bytes(), rhs.as_bytes()) == Ordering::Less
}

fn compare(lhs: &[u8], rhs: &[u8]) -> Ordering {
    // match the first two characters
    match (lhs[0], rhs[0]) {
        (a, b) if a == b => compare(&lhs[1..], &rhs[1..]),
        (_, b']') => Ordering::Greater,
        (b']', _) => Ordering::Less,
        (b'[', _) => {
            let subrhs = [&[rhs[0], b']'], &rhs[1..]].concat();
            compare(&lhs[1..], &subrhs)
        }
        (_, b'[') => {
            let sublhs = [&[lhs[0], b']'], &lhs[1..]].concat();
            compare(&sublhs, &rhs[1..])
        }
        (_, _) => lhs[0].cmp(&rhs[0]),
    }
}

fn process_part1() -> Result<(), Box<dyn Error>> {
    println!("\t\tPart 1");
    let result: i64 = read_to_string("inputs/day13.txt")?
        .replace("10", "A") // do this so we don't have to worry about double digit numbers
        .split("\n\n")
        .enumerate()
        .filter(|(_, pair)| correctorder(pair))
        .map(|(i, _)| i as i64 + 1)
        .sum();

    println!("{}", result);

    Ok(())
}

fn process_part2() -> Result<(), Box<dyn Error>> {
    let mut input = read_to_string("inputs/day13.txt")?
        .replace("10", "A") // do this so we don't have to worry about double digit numbers
        .replace("\n\n", "\n");

    input.push_str("\n[[2]]\n[[6]]");

    // Convert to a vector of strings
    let mut packets = input.lines().collect::<Vec<&str>>();

    // since the compare function uses Ordering, we can simply use it as the comparison function
    // for the sort function
    packets.sort_by(|lhs, rhs| compare(lhs.as_bytes(), rhs.as_bytes()));

    // println!("{:?}", packets);

    let idx_of_2 = packets.iter().position(|&x| x.eq("[[2]]")).unwrap() + 1;
    let idx_of_6 = packets.iter().position(|&x| x.eq("[[6]]")).unwrap() + 1;

    println!("{}", idx_of_2 * idx_of_6);

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("\tDay 13");
    // process_part1()?;
    process_part2()?;
    Ok(())
}

mod test {
    use super::*;

    #[test]
    fn check_packets() {
        let mut lhs: Value = serde_json::from_str("[[1],[2,3,4]]").unwrap();
        let mut rhs: Value = serde_json::from_str("[[1],4]").unwrap();
        assert_eq!(compare(lhs, rhs), true);

        println!("\n");

        lhs = serde_json::from_str("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap();
        rhs = serde_json::from_str("[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap();
        assert_eq!(compare(lhs, rhs), false);

        lhs = serde_json::from_str("[[[]]]").unwrap();
        rhs = serde_json::from_str("[[]]").unwrap();
        assert_eq!(compare(lhs, rhs), false);

        lhs = serde_json::from_str("[9]").unwrap();
        rhs = serde_json::from_str("[[8,7,6]]").unwrap();
        assert_eq!(compare(lhs, rhs), false);

        lhs = serde_json::from_str("[8, 1]").unwrap();
        rhs = serde_json::from_str("[[[9]]]").unwrap();
        assert_eq!(compare(lhs, rhs), true);
    }
}
