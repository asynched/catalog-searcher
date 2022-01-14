use std::io::Write;
use std::io::{stdin, stdout};

pub fn input(text: &str) -> String {
    let mut input = String::new();

    print!("{}", text);

    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}
