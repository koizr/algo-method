//! https://algo-method.com/tasks/224

use std::cmp::max;

fn main() {
    let input = input_vec_i32();
    let a = input[0];
    let b = input[1];

    let answer = (1..=max(a, b))
        .rev()
        .find(|x| a % x == 0 && b % x == 0)
        .unwrap();

    println!("{}", answer);
}

fn input_vec_i32() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.split_whitespace().map(|n| n.parse().unwrap()).collect()
}
