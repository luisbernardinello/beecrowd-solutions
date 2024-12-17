use std::io::{self, BufRead};

// ========================= macros ============================

macro_rules! read { //como na crate text_io
    () => ({
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input).unwrap();
        input.trim().to_string()
    });
}

macro_rules! read_vec {
    () => ({
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input).unwrap();
        input.trim().split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>()
    });
}

// ========================= solution ============================

fn solve() {
    let text: Vec<String> = read_vec!();

    let mut corrected_words = Vec::new();

    for word in text.iter() {

        let corrected_word = if word.len() >= 4 {
            let prefix = word[0..2].to_string();
            if word[2..4].to_string() == prefix {
                word[2..].to_string()
            } else {
                word.clone()
            }
        } else {
            word.clone()
        };

        corrected_words.push(corrected_word);
    }

    println!("{}", corrected_words.join(" "));
}

// ========================= main ============================

fn main() {
    solve();
}
