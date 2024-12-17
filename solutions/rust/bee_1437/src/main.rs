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
        input.trim().chars().collect::<Vec<char>>()
    });
}

// ========================= solution ============================

fn solve() {

    loop {
        let n: usize = read!();
        if n == 0 {
            break;
        }

        let mut face: char = 'N';

        let commands: Vec<char> = read_vec!();

        for command in commands.iter() {
            if *command == 'D' {
                face = match face {
                    'N' => 'L',
                    'L' => 'S',
                    'S' => 'O',
                    'O' => 'N',
                    _ => face,
                }
            } else if *command == 'E' {               
                face = match face {
                    'N' => 'O',
                    'O' => 'S',
                    'S' => 'L',
                    'L' => 'N',
                    _ => face,
                }
            }
        }

        println!("{}", face);
    }
}

// ========================= main ============================

fn main() {
    solve();
}
