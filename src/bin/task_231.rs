//! https://algo-method.com/tasks/231

fn main() {
    let _ = input_vec_i32();
    let a = input_vec_i32();
    let b = input_vec_i32();

    let answer = a
        .iter()
        .flat_map(|&ai| b.iter().map(|&bi| (ai, bi)).collect::<Vec<_>>())
        .filter(|(ai, bi)| ai > bi)
        .count();

    println!("{}", answer);
}

fn input_vec_i32() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.split_whitespace().map(|n| n.parse().unwrap()).collect()
}
