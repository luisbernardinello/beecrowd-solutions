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
        input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<_>>()
    });
}

// ========================= solution ============================

fn solve() {
    loop {
        let line: Vec<usize> = read_vec!();
        let n = line[0];
        let m = line[1];

        if n == 0 && m == 0 {
            break;
        }

        let bilhetes: Vec<usize> = read_vec!();

        let mut freq = vec![0; n + 1];

        for &bilhete in &bilhetes {
            freq[bilhete] += 1;
        }

        let mut repete = 0;
        for &count in &freq {
            if count > 1 {
                repete += 1;
            }
        }

        println!("{}", repete);
    }
}

// ========================= main ============================

fn main() {
    solve();
}
