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

fn count_valid_passphrases(lines: &Vec<String>, anagrams_allowed: bool) -> i32
{
    let mut valid_passphrases = 0;

    for line in lines {
        let mut passphrase: Vec<String> = line.trim().split(" ").map(|s| s.to_owned()).collect();

        if !anagrams_allowed {
            // Anagrams are not allowed, so sort each individual word in the passphrase
            passphrase = passphrase.iter().map(|word| {
                let mut chars: Vec<char> = word.chars().collect();
                chars.sort();
                let s: String = chars.into_iter().collect();
                s
            }).collect();
        }

        // Number of words in the passphrase before removing duplicates
        let len = passphrase.len();

        // Remove all duplicate words (remove consecutive equal items from the sorted passphrase)
        passphrase.sort();
        passphrase.dedup();

        if passphrase.len() == len {
            // Length did not change: passphrase is valid
            valid_passphrases += 1;
        }
    }
    
    valid_passphrases
}

fn main() {
    let lines: Vec<String> = lines_from_file("input/december04.txt").unwrap();
    println!("Part 1: {:#?}", count_valid_passphrases(&lines, true));
    println!("Part 2: {:#?}", count_valid_passphrases(&lines, false));
}
