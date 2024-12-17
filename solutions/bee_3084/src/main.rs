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

fn solve(angles: Vec<i32>) {
    let h_angle = angles[0];
    let m_angle = angles[1];

    let hour = h_angle / 30;
    let minute = m_angle / 6;

    println!("{:02}:{:02}", hour, minute);
}

// ========================= main ============================

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        let mut input = String::new();
        if handle.read_line(&mut input).unwrap() == 0 {
            break;
        }

        let angles: Vec<i32> = input.trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        if angles.len() == 2 {
            solve(angles);
        }
    }
}
