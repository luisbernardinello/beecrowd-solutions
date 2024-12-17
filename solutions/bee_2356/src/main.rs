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

macro_rules! read_vec {
    () => ({
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input).unwrap();
        input.trim().split_whitespace().map(|x| x.to_string()).collect::<Vec<_>>()
    });
}

// ========================= solution ============================

fn solve() -> bool {
    let dna: Option<String> = read!();

    if dna.is_none() {
        return false;
    }

    let dna = dna.unwrap();

    let seq_resist: String = read!().unwrap();

    if dna.contains(&seq_resist) {
        println!("Resistente");
    } else {
        println!("Nao resistente");
    }

    true
}

// ========================= main ============================

fn main() {
    while solve() {}
}
