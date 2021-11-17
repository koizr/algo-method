//! https://algo-method.com/tasks/237

fn main() {
    let n = input_i32();
    let answer = (0..n)
        .map(|_| input_string().chars().collect::<Vec<char>>())
        .filter(|s| palindrome(&s[..]))
        .count();

    println!("{}", answer);
}

/// 与えられた文字列が回文であれば true を返す
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

fn input_i32() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.trim_end().parse().unwrap()
}

fn input_string() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.trim().to_owned()
}
