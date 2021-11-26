//! https://algo-method.com/tasks/305

fn main() {
    let n = input_i32();

    let answer = (1..=n)
        // 最初は 3 個前, 2 個前から現在地まで来ることはないので 0
        // 1 個前からは現在地まで来るので 1 で初期化してある
        .fold((0, 0, 1), |(t, u, v), _| (u, v, t + u + v))
        .2;

    println!("{}", answer);
}

fn input_i32() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.trim_end().parse().unwrap()
}
