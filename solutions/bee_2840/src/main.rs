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

    let linha_1: Vec<f64> = read_vec!();
    let r: f64 = linha_1[0];
    let l: f64 = linha_1[1];

    let pi: f64 = 3.1415;

    let volume_balao: f64 = (4.0 / 3.0) * pi * r.powi(3);

    let quantidade_baloes = (l / volume_balao).floor() as u64;

    println!("{}", quantidade_baloes);
}

// ========================= main ============================

fn main() {
    solve();
}
