use std::io::{self, BufRead};

macro_rules! read {
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
        input.trim().split_whitespace().map(|x| x.parse::<f64>().unwrap()).collect::<Vec<_>>()
    });
}


fn solve() {
    let vec: Vec<f64> = read_vec!();

    let n: f64 = vec[0];
    let x: f64 = vec[1];
    let y: f64 = vec[2];

    let icm = n / (x + y);

    println!("{:.2}", icm);
}

fn main() {
    solve();
}
