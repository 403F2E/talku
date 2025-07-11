use std::io::{Write, stdin, stdout};

pub fn input(guide: &str) -> String {
    print!("{}", guide);
    let _ = stdout().flush();
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Something went wrong while getting input...");
    input.trim().to_string()
}
