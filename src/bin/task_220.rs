//! https://algo-method.com/tasks/220

fn main() {
    let n = input_i32();

    let answer = (1..=n)
        .filter(|&i| indivisible(i, 2) && indivisible(i, 3) && indivisible(i, 5))
        .count();

    println!("{}", answer);
}

fn indivisible(a: i32, b: i32) -> bool {
    a % b != 0
}

fn input_i32() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.trim_end().parse().unwrap()
}
