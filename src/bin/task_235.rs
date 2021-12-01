//! https://algo-method.com/tasks/235

fn main() {
    let input = input_vec_i32();
    let n = input[0];
    let k = input[1];

    let answer = (1..=n).filter(|&x| devisors(x) == k).count();

    println!("{}", answer);
}

fn devisors(n: i32) -> i32 {
    (1..=n).filter(|d| n % d == 0).count() as i32
}

fn input_vec_i32() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.split_whitespace().map(|n| n.parse().unwrap()).collect()
}
