//! https://algo-method.com/tasks/225

fn main() {
    let n = input_i32();

    (1..=n)
        .map(|n| match (n % 3, n % 5) {
            (0, 0) => "FizzBuzz".to_owned(),
            (0, _) => "Fizz".to_owned(),
            (_, 0) => "Buzz".to_owned(),
            _ => n.to_string(),
        })
        .for_each(|answer| println!("{}", answer));
}

fn input_i32() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.trim_end().parse().unwrap()
}
