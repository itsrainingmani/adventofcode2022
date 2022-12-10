use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone)]
struct Arena {
    named: HashMap<String, usize>,
    nodes: Vec<Dir>,
}

impl Arena {
    fn new_node(&mut self, data: Dir) -> usize {
        // Get the next free index
        let next_index = self.nodes.len();

        let data_name = data.name.clone();

        // Push the node into the arena
        self.nodes.push(data);
        self.named.insert(data_name, next_index);

        // Return the node identifier
        next_index
    }

    fn get_parent(&self, cur_id: usize) -> usize {
        if let Some(x) = self.nodes[cur_id].parent {
            x
        } else {
            0
        }
    }

    fn propagate_filesize(&mut self, c_id: usize) {
        let mut cur_id = c_id.clone();
        let mut opt_parent = self.nodes[cur_id].parent;

        while opt_parent != None {
            let nds = &mut self.nodes;
            let cur_fs = nds[cur_id].file_size.clone();

            let mut parent_dir = &mut nds[opt_parent.unwrap()];
            parent_dir.file_size += cur_fs;

            cur_id = opt_parent.unwrap();
            opt_parent = parent_dir.parent;
        }
    }
}

#[derive(Debug, Clone)]
struct Dir {
    name: String,
    dirs: Vec<usize>,
    file_size: u64,
    parent: Option<usize>,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("\nDay 7");

    let file = File::open("../inputs/day7-test2.txt")?;
    let reader = BufReader::new(file);

    let mut mem_arena = Arena {
        named: HashMap::new(),
        nodes: Vec::new(),
    };

    let top_level_dir: Dir = Dir {
        name: "/".to_string(),
        dirs: Vec::new(),
        file_size: 0u64,
        parent: None,
    };
    let mut current_dir_idx: usize = mem_arena.new_node(top_level_dir);

    let mut in_ls: bool = false;

    for line in reader.lines().map(|l| l.unwrap()) {
        // println!("{}", line);
        if line.starts_with("$ cd") {
            in_ls = false;
            let which_dir = (line.split(" ").collect::<Vec<_>>())[2];
            if which_dir == "/" {
                // We are in the root dir
                // println!("Root Dir");
                continue;
            } else if which_dir == ".." {
                // Go back up a dir
                current_dir_idx = mem_arena.get_parent(current_dir_idx);
            } else {
                current_dir_idx = *mem_arena.named.get(which_dir).unwrap();
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
                    let new_dir = Dir {
                        name: dir_name.clone(),
                        dirs: Vec::new(),
                        file_size: 0u64,
                        parent: Some(current_dir_idx),
                    };
                    let new_idx = mem_arena.new_node(new_dir);
                    let current_dir = mem_arena.nodes.get_mut(current_dir_idx).unwrap();
                    current_dir.dirs.push(new_idx);
                } else {
                    // the fs_item should contain a file size and a filename component
                    let file_size = fs_item[0].parse::<u64>().unwrap();

                    let current_dir = mem_arena.nodes.get_mut(current_dir_idx).unwrap();
                    current_dir.file_size += file_size;
                }
            }
        }
    }

    // The root dir will be the first dir in the nodes vec
    let mut dfs_stack: VecDeque<usize> = VecDeque::new();
    let mut visited: HashSet<String> = HashSet::new();

    // Create a mutable memory location with dynamically checked borrow rules
    let mem_arena_rc = RefCell::new(mem_arena);
    let mem_arena_borrowed = &mut mem_arena_rc.borrow_mut();

    dfs_stack.push_back(0);
    while !dfs_stack.is_empty() {
        let cur_idx = dfs_stack.pop_back().unwrap();
        let u = mem_arena_borrowed.nodes[cur_idx].clone();

        if !visited.contains(&u.name) {
            visited.insert(u.name.clone());

            if u.dirs.len() == 0 {
                // this is a leaf node
                // we can propagate the filesizes up the tree
                mem_arena_borrowed.propagate_filesize(cur_idx);
                continue;
            } else {
                for d in &u.dirs {
                    let tmp_dir = &mem_arena_borrowed.nodes[*d];
                    if !visited.contains(&tmp_dir.name) {
                        dfs_stack.push_back(*d);
                    }
                }
            }
        }
    }

    // Filter out dirs with sums greater than 100k and then sum
    let mut fs_total_sum: u64 = 0;
    for n in &mem_arena_borrowed.nodes {
        // println!("{:?}", n);
        if n.file_size <= 100000u64 {
            fs_total_sum += n.file_size;
        }
    }

    println!("{:?}", fs_total_sum);

    // let mut test_cur_id: usize = 2;
    // mem_arena.propagate_filesize(test_cur_id);
    // test_cur_id = 3;
    // mem_arena.propagate_filesize(test_cur_id);

    // println!("{:?} - \n{}", mem_arena, test_cur_id);

    Ok(())
}
