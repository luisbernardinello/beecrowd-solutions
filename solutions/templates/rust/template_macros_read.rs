// This template is made by Luis Bernardinello  <luis.bernardinello@gmail.com>
// GitHub : https://github.com/luisbernardinello

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

    let n: usize = read!();
    let v: Vec<usize> = read_vec!();
    println!("value of n: {}", n);
    println!("vector: {:?}", v);
}

// ========================= main ============================

fn main() {
    let t: usize = read!();
    for _ in 0..t {
        solve();
    }
}
