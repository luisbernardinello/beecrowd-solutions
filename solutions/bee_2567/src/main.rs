use std::io::{self, BufRead};

// ========================= macros ============================

macro_rules! read {
    () => ({
        let mut input = String::new();
        let bytes_read = io::stdin().lock().read_line(&mut input).unwrap();
        if bytes_read == 0 { // EOF
            None
        } else {
            Some(input.trim().parse().unwrap())
        }
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

fn calcula_letalidade(mut vec_virus: Vec<i32>) -> i32 {

    vec_virus.sort();

    let mut soma = 0;
    let mut i = 0;
    let mut j = vec_virus.len() - 1;

    while j > i {
        soma += (vec_virus[j] - vec_virus[i]).abs();
        i += 1;
        j -= 1;
    }

    soma
}

fn solve() -> bool {

    let quantidade_virus: Option<i32> = read!();

    if quantidade_virus.is_none() {
        return false;
    }

    let idade_virus: Vec<i32> = read_vec!();

    let result = calcula_letalidade(idade_virus);

    println!("{}", result);


    true
}

// ========================= main ============================

fn main() {

    while solve() {} 
 

}
