//! https://algo-method.com/tasks/210

fn main() {
    let line1 = input_vec_i32();
    let v = line1[1];

    let nums = input_vec_i32();

    let answer = nums.iter().filter(|&&n| n == v).count();

    println!("{}", answer);
}

fn input_vec_i32() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.split_whitespace().map(|n| n.parse().unwrap()).collect()
}
