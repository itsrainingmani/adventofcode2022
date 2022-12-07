use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    let mut marker_pos: usize = 0;
    let mut marker_set: HashSet<&u8> = HashSet::new();
    for m in input.as_bytes().windows(14) {
        marker_set.clear();
        marker_set = HashSet::from_iter(m.iter());
        if marker_set.len() == 14 {
            marker_pos += 14;
            break;
        } else {
            marker_pos += 1;
        }
    }

    // Answer is 1343 for part a
    println!("{}", marker_pos);
}
