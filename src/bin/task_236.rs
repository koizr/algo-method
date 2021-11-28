//! https://algo-method.com/tasks/236

use std::collections::HashSet;

fn main() {
    let s = input_string();

    let answer = s
        .chars()
        .fold(HashSet::new(), |mut set, c| {
            set.insert(c);
            set
        })
        .len();

    println!("{}", answer);
}

fn input_string() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.trim().to_owned()
}
