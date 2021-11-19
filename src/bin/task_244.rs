//! https://algo-method.com/tasks/244

fn main() {
    let k = {
        let input = input_vec_i32();
        input[1]
    };

    let a2 = input_vec_i32();
    let a1 = {
        let a_len = a2.len();
        a2.clone().into_iter().zip(0..a_len).collect::<Vec<_>>()
    };

    let answer = a1
        .iter()
        .flat_map(|&(ai, i)| a2[(i + 1)..].iter().map(|&bi| ai + bi).collect::<Vec<_>>())
        .filter(|&sum| sum <= k)
        .count();

    println!("{}", answer);
}

fn input_vec_i32() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.split_whitespace().map(|n| n.parse().unwrap()).collect()
}
