//! https://algo-method.com/tasks/261

fn main() {
    let n = input_i32();
    let s = input_string();
    let chars = s.chars().collect::<Vec<_>>();

    let answer = (0..n)
        .flat_map(|x| {
            ((x + 1)..n)
                .map(|y| (chars[x as usize], chars[y as usize]))
                .collect::<Vec<_>>()
        })
        .filter(|(x, y)| x == y)
        .count();

    println!("{}", answer);
}

fn input_string() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.trim().to_owned()
}

fn input_i32() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.trim_end().parse().unwrap()
}
