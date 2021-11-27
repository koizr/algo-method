//! https://algo-method.com/tasks/306

fn main() {
    let input = input_vec_i32();
    // マスの数（ゴールは最後のマス）
    let n = input[0] as usize;
    // 一度に動けるマスの数
    let m = input[1] as usize;
    // それぞれのマスに書かれたコスト
    let a = input_vec_i32();

    let answer = (1..n)
        .fold(vec![0; m], |prev, i| {
            // 現在地に来るための最小コストを算出する
            let cost = {
                let base_cost = a[i];
                let costs = prev
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(p, cost)| {
                        // P マス前（コストの係数）
                        let p = p as i32 + 1;
                        cost + (base_cost * p)
                    })
                    .collect::<Vec<_>>();
                min_in_vec(costs)
            };

            let mut prev = prev[1..].iter().copied().collect::<Vec<_>>();
            prev.push(cost);

            prev
        })
        .last()
        .cloned()
        .unwrap();

    println!("{}", answer);
}

fn min_in_vec(nums: Vec<i32>) -> i32 {
    nums.iter().fold(nums[0], |n, &min| std::cmp::min(n, min))
}

fn input_vec_i32() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.split_whitespace().map(|n| n.parse().unwrap()).collect()
}
