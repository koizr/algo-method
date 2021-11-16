//! https://algo-method.com/tasks/221

fn main() {
    let n = input_i32();

    let answer = (1..=n).filter(divisible(n)).count();

    println!("{}", answer);
}

fn divisible(a: i32) -> impl Fn(&i32) -> bool {
    move |&b| a % b == 0
}

fn input_i32() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.trim_end().parse().unwrap()
}
