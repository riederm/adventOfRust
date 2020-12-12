use core::panic;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;
use regex::Regex;

struct PasswordPolicy {
    range: Range<usize>,
    letter: char,
    password: String,
}

fn main() {

    let file = File::open("data.txt").unwrap();
    let lines = io::BufReader::new(file).lines();

    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();

    let number_of_valid_lines : i32 = lines.map(|l| {
        let line = l.unwrap();
        let groups = re.captures_iter(&line).next().unwrap_or_else(|| panic!("error in line: {}", line));
        let min = groups[1].parse::<usize>().unwrap();
        let max = groups[2].parse::<usize>().unwrap() + (1 as usize);
        PasswordPolicy{
            range: (min..max),
            letter: groups[3].chars().nth(0).unwrap(),
            password: groups[4].to_string()
        }
    }).map(|pp | {
        let count_of_letters = pp.password.chars().filter(|c| c == &pp.letter).count();
        if pp.range.contains(&count_of_letters) {
            1
        } else {
            0
        }
    }).sum();

    println!("{} valid lines", number_of_valid_lines);
    
}
