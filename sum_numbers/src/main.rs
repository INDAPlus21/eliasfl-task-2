use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin(); // Get stdin

    // Define iterator with input lines from stdin
    let mut lines = stdin.lock().lines().map(|line| line.ok().unwrap());

    // Read first line and parse as `usize` integer
    let to_read: usize = lines.next().unwrap().parse().unwrap();

    // Get next line, split by whitespace, parse as `i32` and collect to vector
    let mut numbers: Vec<i32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect();
    // Slice numbers to only get first `to_read` numbers, even if more are entered
    numbers = numbers[0..to_read].to_vec();
    numbers.sort();

    // Find how many number to sum (middle of vector, including centermost element if odd)
    let to_sum = (to_read + 1) / 2;
    // Slice right half, from middle number to end
    let bigger_half = &numbers[(to_read - to_sum)..];

    // Sum `bigger_half` vector
    let sum: i32 = bigger_half.iter().sum();
    println!("{}", sum)
}
