use std::io::{self, BufRead};

// ========================= macros ============================

macro_rules! read { //como na crate text_io
    () => ({
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    });
}

// ========================= solution ============================

fn fibonacci_return(n: usize) -> u64 {
    let mut fib = vec![0u64; n + 1];
    
    fib[0] = 0;
    if n > 0 {
        fib[1] = 1;
    }

    for i in 2..=n {
        fib[i] = fib[i - 1] + fib[i - 2];
    }

    fib[n]
}

fn solve() {
    let n: usize = read!();
    let result = fibonacci_return(n);
    println!("Fib({}) = {}", n, result);
}

// ========================= main ============================

fn main() {
    let t: usize = read!();
    for _ in 0..t {
        solve();
    }
}
