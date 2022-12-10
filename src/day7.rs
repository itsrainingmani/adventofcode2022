use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone)]
struct Dir {
    name: String,
    dirs: Vec<String>,
    dir_size: u64,
    parent: Option<String>,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("\nDay 7 - v2");

    let file = File::open("../inputs/day7.txt")?;
    let reader = BufReader::new(file);

    let mut in_ls: bool = false;
    let mut index_map: HashMap<String, usize> = HashMap::new();
    let mut nodes: Vec<Dir> = Vec::new();
    let mut cur_idx: usize = nodes.len();

    let top_level_dir: Dir = Dir {
        name: "/".to_string(),
        dirs: Vec::new(),
        dir_size: 0u64,
        parent: None,
    };
    nodes.push(top_level_dir);
    index_map.insert("/".to_string(), cur_idx);

    for line in reader.lines().map(|l| l.unwrap()) {
        if line.starts_with("$ cd") {
            in_ls = false;
            let which_dir = (line.split(" ").collect::<Vec<_>>())[2];
            let cur_node = nodes.get(cur_idx).unwrap();
            if which_dir == "/" {
                // We are in the root dir
                println!("Root Dir");
                continue;
            } else if which_dir == ".." {
                // Go back up a dir
                // println!("Go up ^");
                let parent = cur_node.parent.clone();
                match parent {
                    Some(p) => {
                        cur_idx = *index_map.get(&p).unwrap();
                    }
                    None => cur_idx = 0,
                }
            } else {
                // println!("Go into {}", which_dir);
                let dir_name = if cur_node.name == "/" {
                    which_dir.to_string()
                } else {
                    cur_node.name.clone() + "/" + which_dir
                };
                let new_dir = Dir {
                    name: dir_name.clone(),
                    dirs: Vec::new(),
                    dir_size: 0u64,
                    parent: Some(cur_node.name.clone()),
                };
                cur_idx = nodes.len();
                nodes.push(new_dir);
                index_map.insert(dir_name.clone(), cur_idx);
            }
        } else if line.starts_with("$ ls") {
            in_ls = true;
        } else {
            let mut cur_node = nodes.get_mut(cur_idx).unwrap();
            // Now we are exploring a directory
            if in_ls {
                let fs_item: Vec<&str> = line.split(" ").collect();
                if fs_item[0] == "dir" {
                    // the current dir contains a dir
                    let dir_name = if cur_node.name == "/" {
                        fs_item[1].to_string()
                    } else {
                        cur_node.name.clone() + "/" + fs_item[1]
                    };
                    cur_node.dirs.push(dir_name.clone());
                } else {
                    // the fs_item should contain a file size and a filename component
                    let file_size = fs_item[0].parse::<u64>().unwrap();
                    // println!("{}", file_size);
                    cur_node.dir_size += file_size;
                }
            }
        }
    }

    // println!("{:?}", nodes);
    // println!("{:?}", index_map);

    let mut dfs_stack: VecDeque<String> = VecDeque::new();
    let mut visited: HashSet<String> = HashSet::new();

    dfs_stack.push_back("/".to_string());
    while !dfs_stack.is_empty() {
        let cur_dir = dfs_stack.pop_back().unwrap();
        let cur_idx = index_map.get(&cur_dir).unwrap();

        let u = nodes[*cur_idx].clone();
        if !visited.contains(&u.name) {
            visited.insert(u.name.clone());

            if u.dirs.is_empty() {
                // leaf node. propagate filesizes
                let s_dir = u.name.clone();
                let mut cur_id = index_map.get(&s_dir).unwrap();
                let mut parent = nodes[*cur_id].parent.clone();

                while parent != None {
                    let cur_fs = nodes[*cur_id].dir_size;
                    let parent_idx = index_map.get(&parent.clone().unwrap()).unwrap();
                    let mut parent_dir = &mut nodes[*parent_idx];
                    parent_dir.dir_size += cur_fs;

                    cur_id = parent_idx;
                    parent = parent_dir.parent.clone();
                }

                continue;
            } else {
                for child in u.dirs {
                    if !visited.contains(&child) {
                        dfs_stack.push_back(child.clone());
                    }
                }
            }
        }
    }

    let mut fs_total_sum: u64 = 0;
    for n in nodes {
        // println!("{:?}", n);
        if n.dir_size <= 100000u64 {
            fs_total_sum += n.dir_size;
        }
    }
    // 1453349
    println!("\tPart 1  - {:?}", fs_total_sum);

    Ok(())
}
