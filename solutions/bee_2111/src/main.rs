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

// ========================= solution ============================

fn set_column(mtz: &mut [[u8; 7]; 9], col: usize, value: usize) {
    match value {
        0 => mtz[col] = [1, 0, 0, 1, 1, 1, 1],
        1 => mtz[col] = [1, 0, 1, 0, 1, 1, 1],
        2 => mtz[col] = [1, 0, 1, 1, 0, 1, 1],
        3 => mtz[col] = [1, 0, 1, 1, 1, 0, 1],
        4 => mtz[col] = [1, 0, 1, 1, 1, 1, 0],
        5 => mtz[col] = [0, 1, 0, 1, 1, 1, 1],
        6 => mtz[col] = [0, 1, 1, 0, 1, 1, 1],
        7 => mtz[col] = [0, 1, 1, 1, 0, 1, 1],
        8 => mtz[col] = [0, 1, 1, 1, 1, 0, 1],
        9 => mtz[col] = [0, 1, 1, 1, 1, 1, 0],
        _ => {}
    }
}

fn solve() -> bool {
    let n: Option<u32> = read!();

    if n.is_none() {
        return false;
    }

    let n = n.unwrap();
    let mut mtz = [[0u8; 7]; 9];
    let mut aux = 1;

    for i in 0..9 {
        set_column(&mut mtz, i, (n / aux as u32) as usize % 10);
        aux *= 10;
    }

    for j in 0..7 {
        for k in (0..9).rev() {
            print!("{}", mtz[k][j]);
        }
        println!();
        if j == 1 {
            println!("---------");
        }
    }
    println!();

    true
}

// ========================= main ============================

fn main() {
    while solve() {}
}
