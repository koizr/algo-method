//! https://algo-method.com/tasks/238

fn main() {
    let input = input_vec_i32();
    let l = input[0];
    let r = input[1];

    let answer = (l..=r)
        .map(|x| x.to_string().chars().collect::<Vec<char>>())
        .filter(|x| palindrome(&x[..]))
        .count();

    println!("{}", answer);
}

fn input_vec_i32() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.split_whitespace().map(|n| n.parse().unwrap()).collect()
}

fn palindrome(cs: &[char]) -> bool {
    // 両端が一致しているか検査して、一致していれば（回文の可能性があれば）内側も一致しているか再帰的に検査していく。
    // [a, b, c, b, a]
    //    [b, c, b]
    //       [c]
    match cs.len() {
        0 | 1 => true,
        2 => cs[0] == cs[1],
        _ => cs[0] == cs[cs.len() - 1] && palindrome(&cs[1..(cs.len() - 1)]),
    }
}
