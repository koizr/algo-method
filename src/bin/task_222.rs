//! https://algo-method.com/tasks/222

fn main() {
    let n = input_i32();

    let answer = n > 1 && (2..n).all(|i| n % i != 0);

    println!("{}", if answer { "Yes" } else { "No" });
}

fn input_i32() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.trim_end().parse().unwrap()
}
