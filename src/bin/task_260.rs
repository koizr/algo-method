//! https://algo-method.com/tasks/260

use std::{collections::HashMap, ops::Not};

fn main() {
    let n = input_i32();

    let mut strings = HashMap::<String, i32>::new();

    for _ in 0..n {
        let s = input_string();
        let entry = strings.entry(s).or_insert(0);
        *entry += 1;
    }

    let contains = strings
        .iter()
        .filter(|(_, &count)| count > 1)
        .collect::<Vec<_>>()
        .is_empty()
        .not();

    println!("{}", if contains { "Yes" } else { "No" });
}

fn input_string() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.trim().to_owned()
}

fn input_i32() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.trim_end().parse().unwrap()
}
