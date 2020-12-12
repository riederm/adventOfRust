use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

struct PasswordPolicy {
    lower_index: usize, //index of the lower check
    upper_index: usize, //index of the upper check
    letter: u8,         //the letter to check as a byte
    password: String,   //the full password
}

fn main() {
    let lines = io::BufReader::new(File::open("data.txt").unwrap()).lines();

    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();

    let number_of_valid_lines: i32 = lines
        .map(|l| {
            let line = l.unwrap();
            let groups = re.captures_iter(&line).next().unwrap();
            PasswordPolicy {
                lower_index: groups[1].parse::<usize>().unwrap() - 1, //correct for the 0-based index
                upper_index: groups[2].parse::<usize>().unwrap() - 1, //correct for the 0-based index
                letter: groups[3].as_bytes()[0],
                password: groups[4].to_string(),
            }
        })
        .map(|pp| {
            let password_chars = pp.password.as_bytes();
            //only lower-index or upper-index must be equal to letter, not both (xor)
            if (password_chars[pp.lower_index] == pp.letter)
                ^ (password_chars[pp.upper_index] == pp.letter)
            {
                1
            } else {
                0
            }
        })
        .sum();

    println!("{} valid lines", number_of_valid_lines);
}
