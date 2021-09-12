use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|_line| _line.ok().unwrap());

    let to_read = lines.next().unwrap().parse::<usize>().unwrap();

    let mut numbers: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect();
    numbers = numbers[0..to_read].to_vec();
    numbers.sort();

    let middle_index = (to_read as f64 / 2 as f64).ceil() as usize;
    let bigger_half = &numbers[(to_read - middle_index)..];

    let sum: i32 = bigger_half.iter().sum();
    println!("{}", sum)
}
