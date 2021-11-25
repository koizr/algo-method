//! https://algo-method.com/tasks/304

fn main() {
    let n = input_usize();

    let answer = (2..=n).fold((1, 1), |(t, u), _| (u, t + u)).1;

    println!("{}", answer);
}

fn input_usize() -> usize {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.trim_end().parse().unwrap()
}
