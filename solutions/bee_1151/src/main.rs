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

fn fibonacci_seq(n: usize) -> Vec<u64> {
    let mut fib = vec![0u64; n];
    if n > 0 {
        fib[0] = 0;
    }
    if n > 1 {
        fib[1] = 1;
    }
    for i in 2..n {
        fib[i] = fib[i - 1] + fib[i - 2];
    }
    fib
}

fn solve() {
    let n: usize = read!();

    let fib_seq = fibonacci_seq(n);

    let result = fib_seq.iter()
                        .map(|num| num.to_string())
                        .collect::<Vec<String>>()
                        .join(" ");
    
    println!("{}", result);
}

// ========================= main ============================

fn main() {

    solve();
}
