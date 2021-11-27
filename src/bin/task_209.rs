//! https://algo-method.com/tasks/209

use std::{convert::TryInto, io};

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let [_, v]: [i32; 2] = s
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i32>>()
        .as_slice()
        .try_into()
        .unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let mut nums = s.split_whitespace().map(|n| n.parse::<i32>().unwrap());

    let answer = if nums.any(|x| x == v) { "Yes" } else { "No" };

    println!("{}", answer);
}
