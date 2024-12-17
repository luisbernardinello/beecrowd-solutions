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
    let linha_1: Vec<String> = read_vec!();
    
    let n: usize = linha_1[0].parse().unwrap();
    let l: f64 = linha_1[1].parse().unwrap();
    let q: f64 = linha_1[2].parse().unwrap();
    
    let nomes: Vec<String> = read_vec!();

    let total_cuias = (l / q).floor() as usize; 
    let mut quantidade_ultima_cuia = l - (total_cuias as f64 * q);
    let mut indice_ultimo_participante = total_cuias % n;

    if quantidade_ultima_cuia == 0.0 {
        quantidade_ultima_cuia = q;
        indice_ultimo_participante = (total_cuias - 1) % n;
    }

    let nome_ultimo_participante = &nomes[indice_ultimo_participante];

    println!("{} {:.1}", nome_ultimo_participante, quantidade_ultima_cuia);
}

// ========================= main ============================

fn main() {
    solve();
}
