//! https://algo-method.com/tasks/303

use std::cmp::min;

fn main() {
    let n = input_usize();
    let a = input_vec_i32();

    let answer = (2..n)
        .fold((0, a[1]), |(t, u), i| {
            // - 2 つ前からくる場合（ t + a[i] * 2 ）
            // - 1 つ前からくる場合（ u + a[i] ）
            // のどちらかかかる時間の短い方と、そのマスに来るために必要な時間を足して、より短い方をそのマスに到達する際の最短時間として扱う
            (u, min(t + a[i] * 2, u + a[i]))
        })
        .1;

    println!("{}", answer);
}

fn input_vec_i32() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.split_whitespace().map(|n| n.parse().unwrap()).collect()
}

fn input_usize() -> usize {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.trim_end().parse().unwrap()
}
