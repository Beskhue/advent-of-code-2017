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

fn lines_to_rows_and_cols(lines: Vec<String>) -> Result<Vec<Vec<i32>>, std::num::ParseIntError>
{
    lines.iter().map(|line| {
        let nums: Result<Vec<i32>, _> = line.trim().split("\t").map(str::parse).collect();
        nums
    }).collect()

}

fn calc_checksum(spreadsheet: &Vec<Vec<i32>>) -> i32
{
    let mut sum = 0;
    for row in spreadsheet {
        let max = row.iter().max().unwrap();
        let min = row.iter().min().unwrap();
        sum += max - min;
    }
    sum
}

fn calc_evenly_divisible_numbers(spreadsheet: &Vec<Vec<i32>>) -> i32
{
    let mut sum = 0;
    for row in spreadsheet {
        for numerator in row {
            for denominator in row {
                if numerator != denominator && numerator % denominator == 0
                {
                    // denominator evenly divides numerator
                    sum += numerator / denominator
                }
            }
        }
    }
    sum
}

fn main() {
    let lines: Vec<String> = lines_from_file("input/december02.txt").unwrap();
    let rows_and_cols: Vec<Vec<i32>> = lines_to_rows_and_cols(lines).unwrap();
    println!("Part 1: {:#?}", calc_checksum(&rows_and_cols));
    println!("Part 2: {:#?}", calc_evenly_divisible_numbers(&rows_and_cols));
}

