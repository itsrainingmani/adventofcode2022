use std::{
    collections::VecDeque,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    iter::FromIterator,
    ops::Rem,
};

pub fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("../inputs/day10.txt")?;
    let reader = BufReader::new(file).lines().map(|l| l.unwrap());

    let mut instructions = VecDeque::from_iter(reader);

    let mut total_signal_strength: i16 = 0;
    let mut register = 1i16;

    let mut current_cycle: i16 = 1;
    let mut proc_cycle = 0;

    let mut value_to_add = 0;
    let mut cycles_to_check: Vec<i16> = vec![220, 180, 140, 100, 60, 20];
    let mut crt = vec![".".to_string(); 240];
    let crt_width: i16 = 40;
    let mut current_crt_level: i16 = 0;

    while current_cycle <= 240 {
        if proc_cycle == 0 {
            let instr = instructions.pop_front().unwrap_or("noop".to_string());
            if instr.starts_with("noop") {
                value_to_add = 0;
                proc_cycle = 1;
            } else if instr.starts_with("add") {
                value_to_add = instr.split(" ").collect::<Vec<_>>()[1]
                    .parse::<i16>()
                    .unwrap();
                proc_cycle = 2;
            }
        }

        match cycles_to_check.last() {
            Some(x) => {
                if current_cycle == *x {
                    total_signal_strength += register * cycles_to_check.pop().unwrap();
                }
            }
            None => {}
        }

        let crt_index = current_cycle - 1;
        if crt_index > 39 && crt_index.rem(crt_width) == 0 {
            current_crt_level += 1;
        }
        let adj_index = crt_index - (current_crt_level * crt_width);
        let pixel_bounds = vec![register - 1, register, register + 1];
        if pixel_bounds.contains(&adj_index) {
            crt[crt_index as usize] = "#".to_string();
        }

        proc_cycle -= 1;

        if proc_cycle == 0 {
            register += value_to_add;
            value_to_add = 0;
        }

        current_cycle += 1;
    }

    println!("Part 1 - Signal Strength - {}", total_signal_strength);
    // println!("{:?}", crt);

    // EKRHEPUZ
    for (idx, c) in crt.iter().enumerate() {
        if idx > 39 && idx.rem(crt_width as usize) == 0 {
            print!("\n");
        }
        print!("{}", c);
    }

    Ok(())
}
