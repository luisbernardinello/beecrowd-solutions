// This template is made by Luis Bernardinello  <luis.bernardinello@gmail.com>
// GitHub : https://github.com/luisbernardinello

use std::io::{self, BufRead};

// ========================= macro ============================

macro_rules! input { // como na crate proconio
    // caso para n variáveis e vetores
    ($($var:ident : $t:ty),* $(,)?) => {
        $(
            let $var: $t = {
                let mut input = String::new();
                io::stdin().lock().read_line(&mut input).unwrap();
                input.trim().parse::<$t>().unwrap()
            };
        )*
    };

    // caso para leitura de vetores 
    ($var:ident : [$t:ty; $len:expr]) => {
        let $var: Vec<$t> = {
            let mut input = String::new();
            io::stdin().lock().read_line(&mut input).unwrap();
            input
                .trim()
                .split_whitespace()
                .take($len)
                .map(|x| x.parse::<$t>().unwrap())
                .collect()
        };
    };

    // caso para variáveis e vetores misturados
    ($($var:ident : $t:ty),*; $vec_var:ident : [$vec_type:ty; $len:expr]) => {
        $(
            let $var: $t = {
                let mut input = String::new();
                io::stdin().lock().read_line(&mut input).unwrap();
                input.trim().parse::<$t>().unwrap()
            };
        )*
        let $vec_var: Vec<$vec_type> = {
            let mut input = String::new();
            io::stdin().lock().read_line(&mut input).unwrap();
            input
                .trim()
                .split_whitespace()
                .take($len)
                .map(|x| x.parse::<$vec_type>().unwrap())
                .collect()
        };
    };
}

// ========================= solution ============================

fn solve() {

    input! {
        n: usize;
        arr: [usize; n]
    }

    println!("value of n: {}", n);
    println!("vector: {:?}", arr);
}

// ========================= main ============================

fn main() {

    input! {
        t: usize
    }

    for _ in 0..t {
        solve();
    }
}
