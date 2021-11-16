//! https://algo-method.com/tasks/225

fn main() {
    let s = input_string().chars().collect::<Vec<char>>();
    let t = input_string().chars().collect::<Vec<char>>();

    let answer = s.windows(t.len()).any(|cs| cs == &t);

    println!("{}", if answer { "Yes" } else { "No" })
}

#[allow(dead_code)]
fn input_string() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    s.trim().to_owned()
}
