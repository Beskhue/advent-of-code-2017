use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;

fn lines_from_file<P>(filename: P) -> Result<Vec<String>, io::Error>
where
    P: AsRef<Path>,
{
    let file = try!(File::open(filename));
    let buf = std::io::BufReader::new(file);
    buf.lines().collect()
}

fn num_steps_to_exit(instructions: &Vec<i32>, next_offset: fn(i32) -> i32) -> i32 {
    let mut num_steps = 0;
    let mut idx = 0;
    let num_instructions = instructions.len();

    // Make a mutable copy
    let mut instructions_copy = instructions.clone();

    while true {
        num_steps += 1;

        // Get the current instruction
        let instruction = instructions_copy[idx];

        // Set the new instruction
        instructions_copy[idx] = next_offset(instruction);

        // Test if we have exited the instructions
        let next_idx = idx as i32 + instruction;
        if next_idx < 0 || next_idx >= num_instructions as i32 {
            break;
        }
        idx = next_idx as usize;
    }
    num_steps
}

fn main() {
    let lines: Vec<String> = lines_from_file("input/december05.txt").unwrap();
    let instructions: Vec<i32> = lines.into_iter().map(|line| line.trim().parse::<i32>().unwrap()).collect();
    println!("Part 1: {:#?}", num_steps_to_exit(&instructions, |offset| offset+1));
    println!("Part 2: {:#?}", num_steps_to_exit(&instructions, |offset| {
        if offset >= 3 {
            offset - 1
        } else {
            offset + 1
        }
    }));
}
