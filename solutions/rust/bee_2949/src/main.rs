use std::io::{self, BufRead};

// ========================= macros ============================

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
        input.trim().split_whitespace().map(|x| x.to_string()).collect::<Vec<_>>()
    });
}

// ========================= solution ============================

fn solve() {
    let n: usize = read!();

    let mut hobbits = 0;
    let mut humanos = 0;
    let mut elfos = 0;
    let mut anoes = 0;
    let mut magos = 0;

    for _ in 0..n {
        let v: Vec<String> = read_vec!();
        let race_type = &v[1];

        match race_type.as_str() {
            "X" => hobbits += 1,
            "H" => humanos += 1,
            "E" => elfos += 1,
            "A" => anoes += 1,
            "M" => magos += 1,
            _ => (),
        }
    }

    println!("{} Hobbit(s)", hobbits);
    println!("{} Humano(s)", humanos);
    println!("{} Elfo(s)", elfos);
    println!("{} Anao(oes)", anoes);
    println!("{} Mago(s)", magos);
}

// ========================= main ============================

fn main() {
    solve();
}
