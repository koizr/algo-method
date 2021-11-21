//! https://algo-method.com/tasks/259

use std::cmp::max;

fn main() {
    let _ = input_vec_i32();
    let nums = input_vec_i32();
    let len = nums.len();

    let mut answer = 0;

    for x in 0..len {
        for y in (x + 1)..len {
            for z in (y + 1)..len {
                if max3(nums[x], nums[y], nums[z]) == nums[y] {
                    answer += 1;
                }
            }
        }
    }

    println!("{}", answer);
}

fn max3(a: i32, b: i32, c: i32) -> i32 {
    max(max(a, b), c)
}

fn input_vec_i32() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.split_whitespace().map(|n| n.parse().unwrap()).collect()
}
