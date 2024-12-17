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
    let _m: usize = read!();
    let v: Vec<usize> = read_vec!();

    let mut odd_numbers: Vec<usize> = v.into_iter().filter(|&x| x % 2 != 0).collect();

    if odd_numbers.is_empty() {
        println!("");
        return;
    }

    odd_numbers.sort();

    let mut result = Vec::new();
    let mut i = 0;
    let mut j = odd_numbers.len() - 1;

    while i <= j {
        if i == j {
            result.push(odd_numbers[i]);
        } else {
            result.push(odd_numbers[j]);
            result.push(odd_numbers[i]);
        }
        i += 1;
        if j > 0 {
            j -= 1;
        }
    }

    for (index, num) in result.iter().enumerate() {
        if index == result.len() - 1 {
            print!("{}", num);
        } else {
            print!("{} ", num);
        }
    }
    println!(); 
}

// ========================= main ============================

fn main() {
    let t: usize = read!();
    for _ in 0..t {
        solve();
    }
}
