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

macro_rules! read_vec {
    () => ({
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input).unwrap();
        input.trim().split_whitespace().map(|x| x.to_string()).collect::<Vec<_>>()
    });
}

// ========================= solution ============================

fn solve() -> bool {
    let n: Option<String> = read!();

    if n.is_none() {
        return false;
    }

    let cases: usize = n.unwrap().parse().unwrap();

    let mut livros: Vec<String> = Vec::with_capacity(cases);

    for _ in 0..cases {
        let codigo_livro: Vec<String> = read_vec!();
        livros.push(codigo_livro[0].clone());
    }

    livros.sort();

    for livro in livros {
        println!("{}", livro);
    }

    true
}

// ========================= main ============================

fn main() {
    while solve() {}
}
