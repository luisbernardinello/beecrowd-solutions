use std::io::{self, BufRead};

// ========================= macros ============================

macro_rules! read { //como na crate text_io
    () => ({
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    });
}

macro_rules! read_vec {
    () => ({
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input).unwrap();
        input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<_>>()
    });
}

// ========================= solution ============================

fn solve() {

    let primeira_linha: Vec<usize> = read_vec!();

    let n: usize = primeira_linha[0];
    let k: usize = primeira_linha[1];
    let mut alunos: Vec<String> = Vec::new();

    for _ in 0..n {
        let nome_aluno: String = read!();
        alunos.push(nome_aluno);
    }

    alunos.sort();

    println!("{}", alunos[k - 1]);
}

// ========================= main ============================

fn main() {

    solve();

}
