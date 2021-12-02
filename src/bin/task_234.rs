//! https://algo-method.com/tasks/234

fn main() {
    let _ = input_vec_i32();
    let a = input_vec_i32();

    let answer = a.into_iter().filter(|&n| prime(n)).count();

    println!("{}", answer);
}

fn prime(n: i32) -> bool {
    if n < 2 {
        false
    } else {
        (2..n).all(|x| n % x != 0)
    }
}

fn input_vec_i32() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.split_whitespace().map(|n| n.parse().unwrap()).collect()
}
