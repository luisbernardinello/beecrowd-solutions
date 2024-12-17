use std::io::{self, BufRead};

// ========================= macros ============================

macro_rules! read {
    () => ({
        let mut input = String::new();
        let bytes_read = io::stdin().lock().read_line(&mut input).unwrap();
        if bytes_read == 0 { // EOF
            None
        } else {
            Some(input.trim().to_string())
        }
    });
}

// ========================= solution ============================

fn solve() -> bool {
    let mut names = Vec::new();

    while let Some(name) = read!() {
        names.push(name);
    }

    if names.is_empty() {
        return false;
    }

    names.sort_by_key(|name| name.to_lowercase());


    if let Some(last_name) = names.pop() {
        println!("{}", last_name);
    }

    true
}

// ========================= main ============================

fn main() {
    solve();
}
