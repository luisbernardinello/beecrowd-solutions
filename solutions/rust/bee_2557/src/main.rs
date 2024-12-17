use std::io::{self, BufRead};

// ========================= macros ============================

macro_rules! read {
    () => ({
        let mut input = String::new();
        let bytes_read = io::stdin().lock().read_line(&mut input).unwrap();
        if bytes_read == 0 {
            None
        } else {
            Some(input.trim().to_string())
        }
    });
}

// ========================= solution ============================

fn calcula_enigma(formula: &str) -> i32 {
    let partes: Vec<&str> = formula.split(|c| c == '+' || c == '=').collect();
    
    let r = partes[0].trim();
    let l = partes[1].trim();
    let j = partes[2].trim();

    if r == "R" {
        let l_val: i32 = l.parse().unwrap();
        let j_val: i32 = j.parse().unwrap();
        return j_val - l_val;
    } else if l == "L" {
        let r_val: i32 = r.parse().unwrap();
        let j_val: i32 = j.parse().unwrap();
        return j_val - r_val;
    } else if j == "J" {
        let r_val: i32 = r.parse().unwrap();
        let l_val: i32 = l.parse().unwrap();
        return r_val + l_val;
    }

    0
}

fn solve() -> bool {
    let enigma: Option<String> = read!();

    if enigma.is_none() {
        return false;
    }

    let result = calcula_enigma(&enigma.unwrap());

    println!("{}", result);

    true
}

// ========================= main ============================

fn main() {
    while solve() {}
}
