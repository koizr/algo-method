//! https://algo-method.com/tasks/245

fn main() {
    let input = input_vec_i32();
    let l = input[0];
    let r = input[1];

    let nums1 = l..=r;
    let nums2 = nums1.clone().collect::<Vec<_>>();

    let answer = nums1
        .enumerate()
        .flat_map(|(i, n1)| {
            nums2[(i + 1)..]
                .iter()
                .map(|n2| (n1, n2))
                .collect::<Vec<_>>()
        })
        .filter(|&(n1, n2)| n1 % 10 == n2 % 10)
        .count();

    println!("{}", answer);
}

fn input_vec_i32() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.split_whitespace().map(|n| n.parse().unwrap()).collect()
}
