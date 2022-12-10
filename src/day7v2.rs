use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("\nDay 7 - v2");

    let file = File::open("../inputs/day7-test2.txt")?;
    let reader = BufReader::new(file);

    let mut in_ls: bool = false;

    let mut fs_map: HashMap<String, u64> = HashMap::new();
    let mut parent_map: HashMap<String, usize> = HashMap::new();
    let mut children_map: HashMap<String, Vec<usize>> = HashMap::new();
    let mut nodes: Vec<String> = Vec::new();

    let mut cur_idx: usize = nodes.len();

    for line in reader.lines().map(|l| l.unwrap()) {
        if line.starts_with("$ cd") {
            in_ls = false;
            let which_dir = (line.split(" ").collect::<Vec<_>>())[2];
            if which_dir == "/" {
                // We are in the root dir
                println!("Root Dir");
                fs_map.insert(which_dir.to_string(), 0);
                continue;
            } else if which_dir == ".." {
                // Go back up a dir
                println!("Go up ^");
            } else {
                println!("Go into {}", which_dir);
            }
        } else if line.starts_with("$ ls") {
            in_ls = true;
        } else {
            // Now we are exploring a directory
            if in_ls {
                let fs_item: Vec<&str> = line.split(" ").collect();
                if fs_item[0] == "dir" {
                    // the current dir contains a dir
                    let dir_name = fs_item[1].to_string();
                    println!("dir {}", dir_name);
                } else {
                    // the fs_item should contain a file size and a filename component
                    let file_size = fs_item[0].parse::<u64>().unwrap();
                    println!("{}", file_size);
                }
            }
        }
    }

    Ok(())
}
