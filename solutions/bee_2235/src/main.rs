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

fn calcula_viagem(a: isize, b: isize, c: isize) {

    if a - b == 0 ||
       a - c == 0 ||
       b - c == 0 ||
       (a + b) - c == 0 ||
       (b + c) - a == 0 ||
       (a + c) - b == 0 {
        println!("S");
    } else {
        println!("N");
    }
}


fn solve() {

    let vec: Vec<isize> = read_vec!();
    let a:isize = vec[0];
    let b:isize = vec[1];
    let c:isize = vec[2];

    calcula_viagem(a, b, c);


}

// ========================= main ============================

fn main() {

    solve();

}
