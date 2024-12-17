use std::io::{self, BufRead};

// ========================= macros ============================

macro_rules! read {
    () => ({
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input).unwrap();
        input.trim().to_string()
    });
}

macro_rules! read_vec {
    () => ({
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input).unwrap();
        input.trim().split_whitespace().map(|x| x.to_string()).collect::<Vec<_>>()
    });
}

// ========================= enum ============================

#[derive(Debug, Clone)]
enum PaisMedalha {
    Pais(String),
    Ouro(i32),
    Prata(i32),
    Bronze(i32),
}

fn get_medalha(pais: &Vec<PaisMedalha>) -> (String, i32, i32, i32) {
    let mut nome = String::new();
    let mut ouro = 0;
    let mut prata = 0;
    let mut bronze = 0;

    for item in pais {
        match item {
            PaisMedalha::Pais(p) => nome = p.clone(),
            PaisMedalha::Ouro(o) => ouro = *o,
            PaisMedalha::Prata(p) => prata = *p,
            PaisMedalha::Bronze(b) => bronze = *b,
        }
    }

    (nome, ouro, prata, bronze)
}

// ========================= solution ============================

fn calcula_ordem(mut paises: Vec<Vec<PaisMedalha>>) -> Vec<Vec<PaisMedalha>> {

    paises.sort_by(|a, b| {
        let (nome_a, ouro_a, prata_a, bronze_a) = get_medalha(a);
        let (nome_b, ouro_b, prata_b, bronze_b) = get_medalha(b);
        
        if ouro_a != ouro_b {
            ouro_b.cmp(&ouro_a)
        } else if prata_a != prata_b {
            prata_b.cmp(&prata_a)
        } else if bronze_a != bronze_b {
            bronze_b.cmp(&bronze_a)
        } else {
            nome_a.cmp(&nome_b)
        }
    });

    paises
}

fn solve() {

    let n: usize = read!().parse().unwrap();
    
    let mut paises: Vec<Vec<PaisMedalha>> = Vec::with_capacity(n);

    for _ in 0..n {
        let linha: Vec<String> = read_vec!();
        let nome = linha[0].clone();
        let ouro: i32 = linha[1].parse().unwrap();
        let prata: i32 = linha[2].parse().unwrap();
        let bronze: i32 = linha[3].parse().unwrap();

        paises.push(vec![
            PaisMedalha::Pais(nome),
            PaisMedalha::Ouro(ouro),
            PaisMedalha::Prata(prata),
            PaisMedalha::Bronze(bronze),
        ]);
    }

    let paises_ordenados = calcula_ordem(paises);

    for pais in paises_ordenados {
        let (nome, ouro, prata, bronze) = get_medalha(&pais);
        println!("{} {} {} {}", nome, ouro, prata, bronze);
    }
}

// ========================= main ============================

fn main() {
    solve();
}
