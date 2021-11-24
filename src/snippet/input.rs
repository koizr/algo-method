#[allow(dead_code)]
fn input_string() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.trim().to_owned()
}

#[allow(dead_code)]
fn input_i32() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.trim_end().parse().unwrap()
}

#[allow(dead_code)]
fn input_vec_i32() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.split_whitespace().map(|n| n.parse().unwrap()).collect()
}

#[allow(dead_code)]
fn input_i32_as_bits() -> i32 {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    i32::from_str_radix(&s, 2).unwrap()
}
