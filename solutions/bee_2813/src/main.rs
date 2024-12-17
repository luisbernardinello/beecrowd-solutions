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
    let cases: usize = read!();
    
    let mut umbrellas_home = 0;
    let mut umbrellas_work = 0;
    let mut at_home = 0;
    let mut at_work = 0;

    for _ in 0..cases {
        let weather: Vec<String> = read_vec!();
        let (morning, evening) = (weather[0].as_str(), weather[1].as_str());

        if morning == "chuva" {
            if at_home > 0 {
                at_home -= 1;
            } else {
                umbrellas_home += 1;
            }
            at_work += 1;
        }

        if evening == "chuva" {
            if at_work > 0 {
                at_work -= 1;
            } else {
                umbrellas_work += 1;
            }
            at_home += 1;
        }
    }

    println!("{} {}", umbrellas_home, umbrellas_work);
}

// ========================= main ============================

fn main() {
    solve();
}