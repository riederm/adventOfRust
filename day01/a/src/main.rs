use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    const TARGET: i32 = 2020;
    let file = File::open("data.txt").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut numbers: Vec<i32> = lines
        .map(|line| line.unwrap())
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    numbers.sort();

    let mut bottom = 0;
    let mut mid = 1;
    let mut top = numbers.len() - 1;

    let mut current = numbers[bottom] + numbers[mid] + numbers[top];

    while current != TARGET {
        if current < TARGET {
            if numbers[bottom+1] - numbers[bottom] < numbers[mid+1] - numbers[mid] {
                bottom += 1;
            } else{
                mid += 1;
            }
        } else if current > TARGET {
            if numbers[top] - numbers[top-1] < numbers[mid] - numbers[mid-1] {
                top -= 1;
            } else{
                mid -= 1;
            }
        }
        current = numbers[bottom] + numbers[mid] + numbers[top];
        println!(
            "result: {} + {} + {} = {} ",
            bottom,
            mid,
            top, 
            numbers[bottom] + numbers[mid] + numbers[top]
        );
    }
    println!(
        "result: {} + {} = {} ",
        numbers[bottom], numbers[top], current
    );
}
