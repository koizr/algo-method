//! https://algo-method.com/tasks/323

fn main() {
    let input = input_vec_usize();
    // マスの数（ゴールは最後のマス）
    let n = input[0];
    // 移動可能なマスの数が書かれたダイス（[dice.len()]面ダイスと言える）
    let dice = input_vec_usize();

    // 到達可能なマスを true にしていく
    let mut route = vec![false; n + 1];
    // 最初のマスには最初から到達しているので必ず true になる
    route[0] = true;
    for i in 1..=n {
        for roll in &dice {
            // 現在地に到達する前のマスを探す
            let prev = (i as i32) - (*roll as i32);
            if 0 <= prev && prev < (n as i32) && route[prev as usize] {
                route[i] = true;
            }
        }
    }

    let answer = if route.last().copied().unwrap() {
        "Yes"
    } else {
        "No"
    };

    println!("{}", answer);
}

fn input_vec_usize() -> Vec<usize> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.split_whitespace().map(|n| n.parse().unwrap()).collect()
}
