use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

// i and j are the current indices of the tree
fn is_tree_visible(tmap: &Vec<Vec<u8>>, i: usize, j: usize) -> bool {
    let cur_tree_height: u8 = tmap[i][j];
    let mut is_vis_left: bool = true;
    let mut is_vis_right: bool = true;
    let mut is_vis_up: bool = true;
    let mut is_vis_bottom: bool = true;
    let cur_row = &tmap[i];

    let mut cur_col: Vec<u8> = Vec::new();
    for y in tmap {
        cur_col.push(y[j]);
    }

    // check from the left of the current row
    for r in 0..j {
        if cur_row[r] >= cur_tree_height {
            is_vis_left = false;
        }
    }
    for r in (j + 1)..cur_row.len() {
        if cur_row[r] >= cur_tree_height {
            is_vis_right = false;
        }
    }
    for c in 0..i {
        if cur_col[c] >= cur_tree_height {
            is_vis_up = false;
        }
    }
    for c in (i + 1)..cur_col.len() {
        if cur_col[c] >= cur_tree_height {
            is_vis_bottom = false;
        }
    }

    is_vis_left || is_vis_right || is_vis_up || is_vis_bottom
}

fn calculate_scenic_score(tmap: &Vec<Vec<u8>>, i: usize, j: usize) -> u32 {
    let cur_tree_height: u8 = tmap[i][j];
    let mut view_dist_left: u32 = 0;
    let mut view_dist_right: u32 = 0;
    let mut view_dist_up: u32 = 0;
    let mut view_dist_bottom: u32 = 0;
    let cur_row = &tmap[i];

    let mut cur_col: Vec<u8> = Vec::new();
    for y in tmap {
        cur_col.push(y[j]);
    }

    for r in (0..j).rev() {
        // println!("left - {}", r);
        view_dist_left += 1;
        if cur_row[r] >= cur_tree_height {
            break;
        }
    }
    for r in (j + 1)..cur_row.len() {
        // println!("right - {}", r);
        view_dist_right += 1;
        if cur_row[r] >= cur_tree_height {
            break;
        }
    }
    for c in (0..i).rev() {
        // println!("up - {}", c);
        view_dist_up += 1;
        if cur_col[c] >= cur_tree_height {
            break;
        }
    }
    for c in (i + 1)..cur_col.len() {
        // println!("bottom - {}", c);
        view_dist_bottom += 1;
        if cur_col[c] >= cur_tree_height {
            break;
        }
    }

    view_dist_up * view_dist_bottom * view_dist_left * view_dist_right
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("\nDay 8");

    let file = File::open("../inputs/day8.txt")?;
    let reader = BufReader::new(file);

    let mut tree_map: Vec<Vec<u8>> = Vec::new();

    for (_, line) in reader.lines().map(|l| l.unwrap()).enumerate() {
        let mut temp: Vec<u8> = Vec::new();
        let l_c = line.chars().map(|c| c.to_string().parse::<u8>().unwrap());

        for x in l_c {
            temp.push(x);
        }

        tree_map.push(temp);
    }

    let mut number_of_visible_trees: u32 = 0;
    let mut max_scenic_score: u32 = u32::MIN;

    for i in 1..(tree_map[0].len() - 1) {
        for j in 1..(tree_map.len() - 1) {
            let cur_scenic_score = calculate_scenic_score(&tree_map, i, j);
            if cur_scenic_score > max_scenic_score {
                max_scenic_score = cur_scenic_score;
            }
            if is_tree_visible(&tree_map, i, j) {
                number_of_visible_trees += 1;
            }
        }
    }

    let grid_size = tree_map.len() as u32;
    number_of_visible_trees += grid_size + 2 * (grid_size - 1) + (grid_size - 2);
    println!("{}", number_of_visible_trees);
    println!("{}", max_scenic_score);

    Ok(())
}
