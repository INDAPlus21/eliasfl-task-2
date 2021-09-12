use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    // Define iterator with input lines from stdin
    let mut lines = stdin.lock().lines().map(|line| line.ok().unwrap());

    // Save number of names to get
    let n_names: usize = lines
        .next()
        .expect("Could not read line")
        .parse()
        .expect("First line is not a number");

    // Take `n` first names from lines iterator
    let first_names: Vec<String> = lines.by_ref().take(n_names).collect();
    // Take `n` last names from lines iterator
    let last_names: Vec<String> = lines.take(n_names).collect();

    let mut names: Vec<String> = first_names
        .iter()
        // Zip names to get tuple with (first_name, last_name)
        .zip(last_names.iter())
        // Concatenate first and last name from zipped tuple
        .map(|(fname, lname)| format!("{} {}", fname, lname))
        .collect();
    names.sort();
    // Remove adjacent duplicates
    names.dedup();
    let unique_names = names.len();

    println!("{}", unique_names);
}
