//! https://algo-method.com/tasks/302

fn main() {
    let input = input_vec_i32();
    let n = input[0];
    let x = input[1];
    let y = input[2];

    let answer = (2..n).fold((x, y), |(x, y), _| (y, (x + y) % 100)).1;

    println!("{}", answer);
}

fn input_vec_i32() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.split_whitespace().map(|n| n.parse().unwrap()).collect()
}
