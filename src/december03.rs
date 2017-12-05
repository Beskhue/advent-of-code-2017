use std::{i32};
use std::collections::HashMap;

fn abs(x: i32) -> i32 {
    x.abs()
}

fn calc_steps_to_port_brute_force(data_location: i32) -> i32
{
    // States:
    // 0: grow right
    // 1: grow up
    // 2: grow left
    // 3: grow down
    let mut state = 3;

    let mut x = 0;
    let mut y = 0;
    let mut cur_location = 1;
    let mut cur_radius = 1;
    while cur_location < data_location {
        let state_ = state % 4;
        
        // Find the next position
        if state_ == 0 {
            // Grow right
            if abs(x + 1) < cur_radius {
                x += 1;
                cur_location += 1;
            } else {
                state += 1;
            }
        } else if state_ == 1 {
            // Grow up
            if abs(y + 1) < cur_radius {
                y += 1;
                cur_location += 1;
            } else {
                state += 1;
            }
        } else if state_ == 2 {
            // Grow left
            if abs(x - 1) < cur_radius { 
                x -= 1;
                cur_location += 1;
            } else {
                state += 1;
            }
        } else if state_ == 3 {
            // Grow down
            if abs(y - 1) < cur_radius {
                y -= 1;
                cur_location += 1;
            } else {
                state += 1;
                cur_radius += 1;
            }
        }
    }

    // Calculate the manhattan distance to the center
    abs(x) + abs(y)
}

fn find_first_written_value_larger_than(threshold: i32) -> i32
{
    // States:
    // 0: grow right
    // 1: grow up
    // 2: grow left
    // 3: grow down
    let mut state = 3;

    // A hash map emulating the infinite two-dimensional grid data structure,
    // from data locations (x, y) : (i32, i32) to data content (i32)
    let mut memory: HashMap<(i32, i32), i32> = HashMap::new();
    memory.insert((0,0), 1);

    let mut x = 0;
    let mut y = 0;
    let mut cur_location = 1;
    let mut cur_radius = 1;
    while true {
        let state_ = state % 4;
        
        // Find the next position
        let mut success = true;
        if state_ == 0 {
            // Grow right
            if abs(x + 1) < cur_radius {
                x += 1;
                cur_location += 1;
            } else {
                state += 1;
                success = false;
            }
        } else if state_ == 1 {
            // Grow up
            if abs(y + 1) < cur_radius {
                y += 1;
                cur_location += 1;
            } else {
                state += 1;
                success = false;
            }
        } else if state_ == 2 {
            // Grow left
            if abs(x - 1) < cur_radius { 
                x -= 1;
                cur_location += 1;
            } else {
                state += 1;
                success = false;
            }
        } else if state_ == 3 {
            // Grow down
            if abs(y - 1) < cur_radius {
                y -= 1;
                cur_location += 1;
            } else {
                state += 1;
                cur_radius += 1;
                success = false;
            }
        }

        if success {
            // Find the new data value
            let data = 
                      *memory.get(&(x+1, y  )).unwrap_or(&0)
                    + *memory.get(&(x+1, y+1)).unwrap_or(&0)
                    + *memory.get(&(x  , y+1)).unwrap_or(&0)
                    + *memory.get(&(x-1, y+1)).unwrap_or(&0)
                    + *memory.get(&(x-1, y  )).unwrap_or(&0)
                    + *memory.get(&(x-1, y-1)).unwrap_or(&0)
                    + *memory.get(&(x  , y-1)).unwrap_or(&0)
                    + *memory.get(&(x+1, y-1)).unwrap_or(&0);

            // Insert the data value into memory
            memory.insert((x, y), data);

            if data > threshold {
                return data;
            }
        }
    };

    -1
}

fn main() {
    println!("Part 1: {:#?}", calc_steps_to_port_brute_force(265149));
    println!("Part 2: {:#?}", find_first_written_value_larger_than(265149));
}

