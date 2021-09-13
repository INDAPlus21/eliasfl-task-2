use std::collections::HashSet;
use std::io::{stdin, BufRead};

fn main() {
    let _input = stdin();
    let mut input = _input.lock();

    let mut n_names = String::new();
    input.read_line(&mut n_names);
    let n_names: usize = n_names.trim().parse().unwrap();

    let mut names = Vec::with_capacity(n_names);
    for _ in 0..n_names {
        let mut tuple = (String::with_capacity(20), String::with_capacity(20));
        input.read_line(&mut tuple.0);
        names.push(tuple);
    }
    for i in 0..n_names {
        input.read_line(&mut names[i].1);
    }

    let mut set: HashSet<(String, String)> = HashSet::with_capacity(n_names);
    set.extend(names);

    // let set: HashSet<_> = names.drain(..n_names).collect();

    println!("{}", set.len());
}
