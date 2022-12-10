use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("\nDay 7 - v2");

    let file = File::open("../inputs/day7.txt")?;
    let reader = BufReader::new(file);

    let mut current_path: Vec<String> = Vec::new();
    let mut dir_sizes: HashMap<String, u32> = HashMap::new();

    let mut in_ls: bool = false;
    for line in reader.lines().map(|l| l.unwrap()) {
        if line.starts_with("$ cd") {
            in_ls = false;
            let mut which_dir = (line.split(" ").collect::<Vec<_>>())[2];
            if which_dir == ".." {
                // Go back up a dir
                current_path.pop();
            } else {
                // println!("Go into {}", which_dir);
                if which_dir == "/" {
                    which_dir = "root";
                }
                current_path.push(which_dir.to_string());
                let path_str = current_path.join("/");
                if !dir_sizes.contains_key(&path_str) {
                    dir_sizes.insert(path_str, 0);
                }
            }
        } else if line.starts_with("$ ls") {
            in_ls = true;
        } else {
            // Now we are exploring a directory
            if in_ls {
                let fs_item: Vec<&str> = line.split(" ").collect();
                if fs_item[0] != "dir" {
                    let file_size = fs_item[0].parse::<u32>().unwrap();
                    let mut dirs_to_update: Vec<String> = Vec::new();
                    for dir in &current_path {
                        dirs_to_update.push(dir.clone());
                        let update_path = dirs_to_update.join("/");
                        dir_sizes
                            .entry(update_path)
                            .and_modify(|size| *size += file_size);
                    }
                }
            }
        }
    }

    println!("{:?}", current_path);

    let space_needed = 30_000_000 - (70_000_000 - dir_sizes.get("root").unwrap());
    let dir_to_delete = dir_sizes
        .iter()
        .filter(|(_, &size)| size >= space_needed)
        .min_by(|a, b| a.1.cmp(&b.1))
        .map(|(_, &size)| size)
        .unwrap();
    println!("Directory to Delete - {}", dir_to_delete);

    Ok(())
}
