use std::collections::HashSet;

pub fn main() {
    println!("\nDay 6");
    let input = include_str!("../inputs/day6.txt");

    let sizes: [usize; 2] = [4, 14];

    for ele in sizes {
        let mut marker_pos: usize = 0;
        let mut marker_set: HashSet<&u8> = HashSet::new();
        for m in input.as_bytes().windows(ele) {
            marker_set.clear();
            marker_set = HashSet::from_iter(m.iter());
            if marker_set.len() == ele {
                marker_pos += ele;
                break;
            } else {
                marker_pos += 1;
            }
        }

        // Answer is 1343 for part a
        // Answer is 2193 for part b
        println!("\t{}", marker_pos);
    }
}
