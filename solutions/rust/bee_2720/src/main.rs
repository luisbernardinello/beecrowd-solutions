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

fn calcula_volume(altura: usize, largura: usize, comprimento: usize) -> usize {
    altura * largura * comprimento
}

fn solve() {

    let linha: Vec<usize> = read_vec!();
    let n = linha[0];
    let k = linha[1];

    let mut presentes: Vec<(usize, usize)> = Vec::with_capacity(n);

    for _ in 0..n {
        let v: Vec<usize> = read_vec!();
        let id = v[0];
        let altura = v[1];
        let largura = v[2];
        let comprimento = v[3];

        let volume = calcula_volume(altura, largura, comprimento);

        presentes.push((id, volume));
    }

    // primeiro ordena por volume (decrescente), depois por id (crescente)
    presentes.sort_by(|a, b| {
        let volume_a = a.1;
        let volume_b = b.1;
        if volume_a != volume_b {
            volume_b.cmp(&volume_a)
        } else {
            a.0.cmp(&b.0)
        }
    });

    let mut ids: Vec<usize> = Vec::with_capacity(k);
    for i in 0..k {
        ids.push(presentes[i].0);
    }

    ids.sort();

    for (i, id) in ids.iter().enumerate() {
        if i == 0 {
            print!("{}", id);
        } else {
            print!(" {}", id);
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
