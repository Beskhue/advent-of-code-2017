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

fn redistribute_once(memory: &Vec<i32>) -> Vec<i32> {
    if memory.len() == 0 {
        return Vec::new();
    }

    // Find the index and the number of blocks of the memory position with the
    // greatest number of blocks (use min_by an a flipped comparator, because
    // ties should be won by the lowest-numbered memory position)
    let (mut idx, &blocks_) = memory.iter().enumerate().min_by(|&(_, blocks1), &(_, blocks2)| blocks2.cmp(blocks1)).unwrap();

    // There might be a better way to obtain the blocks variable mutably...
    let mut blocks = blocks_;

    // Copy the memory into a mutable memory structure
    let mut new_memory: Vec<i32> = memory.iter().cloned().collect();

    // Redistribute blocks
    let banks = new_memory.len();

    new_memory[idx] = 0;
    while blocks > 0 {
        idx += 1;
        new_memory[idx % banks] += 1;
        blocks -= 1;
    }
    new_memory
}

fn redistribute_until_loop_detected(mut memory: Vec<i32>, return_size_of_loop: bool) -> i32 {
    let mut historical_memory: Vec<Vec<i32>> = Vec::new();

    let mut steps = 0;
    while true {
        steps += 1;

        // Calculate next memory
        let next_memory = redistribute_once(&memory);

        // Add previous memory to historical memories
        historical_memory.push(memory);

        memory = next_memory;

        // Test if new memory has already been encountered, and if so: break or
        // return the size of the loop
        match historical_memory.iter().position(|ref mem| mem == &&memory) {
            None => (),
            Some(idx) => if return_size_of_loop {
                    return steps - idx as i32;
                } else {
                    return steps;
                }
        };
    }
    
    -1
}

fn main() {
    // Ugly (but functional) way to read input to a vector of i32s
    let lines: Vec<String> = lines_from_file("input/december06.txt").unwrap();
    let memory: Vec<i32> = lines[0].trim().split("\t").map(|num| num.parse::<i32>().unwrap()).collect();

    println!("Part 1: {:#?}", redistribute_until_loop_detected(memory.iter().cloned().collect(), false));
    println!("Part 2: {:#?}", redistribute_until_loop_detected(memory, true));
}
