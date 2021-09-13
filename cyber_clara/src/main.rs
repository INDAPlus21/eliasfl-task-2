use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() {
    let _input = stdin();
    let mut input = _input.lock();
    let mut linestr = String::new();
    input.read_to_string(&mut linestr).unwrap();
    let mut lines = linestr.lines();

    let n_names: usize = lines.next().unwrap().trim().parse().unwrap();

    let names: Vec<&str> = lines.into_iter().collect();
    let fnames = &names[..n_names];
    let lnames = &names[n_names..];

    let mut set = HashSet::with_capacity(n_names);
    for i in 0..n_names {
        set.insert((fnames[i], lnames[i]));
    }

    println!("{}", set.len());
}
