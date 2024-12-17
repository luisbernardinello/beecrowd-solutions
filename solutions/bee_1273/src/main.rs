use std::io::{self, BufRead};

// ========================= macros ============================

macro_rules! read { 
    () => ({
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    });
}


// ========================= solution ============================

fn solve() {
    let mut first_case = true;

    loop {
        let n: usize = read!();

        if n == 0 {
            break;
        }

        if !first_case {
            println!();
        }
        first_case = false;


        let mut words: Vec<String> = Vec::new();
        let mut max_length = 0;

        for _ in 0..n {
            let word: String = read!();
            max_length = max_length.max(word.len());
            words.push(word);
        }

        for word in words {
            println!("{:>width$}", word, width = max_length);
        }
    }
}

// ========================= main ============================

fn main() {
    solve();
}
