use std::io::{self, BufRead};
use std::collections::HashMap;


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

macro_rules! read_char_int {
    () => ({
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let runa = parts[0].chars().next().unwrap();
        let valor = parts[1].parse::<i32>().unwrap(); 
        (runa, valor)
    });
}

fn solve() {
    let vec: Vec<i32> = read_vec!();
    let n = vec[0] as usize;
    let g = vec[1];

    let mut runas = HashMap::new();

    for _ in 0..n {
        let (runa, valor) = read_char_int!();
        runas.insert(runa, valor);
    }

    let _x: usize = read!();

    let mut amizade_total = 0;
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input).unwrap();
    for runa in input.trim().split_whitespace() {
        let runa_char = runa.chars().next().unwrap();
        amizade_total += runas.get(&runa_char).unwrap_or(&0);
    }

    println!("{}", amizade_total);

    if amizade_total >= g {
        println!("You shall pass!");
    } else {
        println!("My precioooous");
    }
}

fn main() {
    solve();
}
