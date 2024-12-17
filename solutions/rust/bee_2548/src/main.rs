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

fn solve() -> bool {

    let nm: Vec<usize> = read_vec!();

    if nm.len() < 2 { // retorna falso e sai do loop
        return false;
    }

    let n = nm[0];
    let m = nm[1];

    let valores: Vec<usize> = read_vec!();

    let mut soma = 0;
    for i in 1..=m {
        soma += valores[n - i];
    }

    println!("{}", soma);

    true
}

// ========================= main ============================

fn main() {

    while solve() {} //end of file
 

}
