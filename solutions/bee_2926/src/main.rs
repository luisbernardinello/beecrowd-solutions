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

    let repete_a: usize = read!();

    //(clojure) para gerar a string repetida
    let gera_string = || {
        let mut result = String::new();
        for _ in 0..repete_a {
            result.push('a');
        }
        result
    };
 

    println!("Ent{}o eh N{}t{}l!", gera_string(), gera_string(), gera_string());
}

fn solve_2() {

    let a: usize = read!();

    // cria string repetida usando um iterador e o metodo collect::<String>()
    println!("Ent{}o eh N{}t{}l!", 
        (0..a).map(|_| "a").collect::<String>(),
        (0..a).map(|_| "a").collect::<String>(),
        (0..a).map(|_| "a").collect::<String>()
    );
}


// ========================= main ============================

fn main() {
    solve();
}
