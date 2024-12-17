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


fn calcula_vencedor(pokemon_dadriel: Vec<usize>, pokemon_guarte: Vec<usize>, bonus_rodada: usize) {
    // [0] -> ataque;
    // [1] -> defesa;
    // [2] -> level treinador;

    let mut valor_golpe_dadriel = (pokemon_dadriel[0] + pokemon_dadriel[1]) / 2;
    if pokemon_dadriel[2] % 2 == 0 {
        valor_golpe_dadriel += bonus_rodada;
    }

    let mut valor_golpe_guarte = (pokemon_guarte[0] + pokemon_guarte[1]) / 2;
    if pokemon_guarte[2] % 2 == 0 {
        valor_golpe_guarte += bonus_rodada;
    }

    if valor_golpe_dadriel > valor_golpe_guarte {
        println!("Dabriel");
    } else if valor_golpe_guarte > valor_golpe_dadriel {
        println!("Guarte");
    } else {
        println!("Empate");
    }
}

fn solve() {

    let valor_bonus: usize = read!();

    let vec_pokemon_dadriel: Vec<usize> = read_vec!();
    let vec_pokemon_guarte: Vec<usize> = read_vec!();

    calcula_vencedor(vec_pokemon_dadriel, vec_pokemon_guarte, valor_bonus);
}

// ========================= main ============================

fn main() {
    let t: usize = read!();
    for _ in 0..t {
        solve();
    }
}
