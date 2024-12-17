fn main() {
    println!("Hello, world!");
}
use std::io::{self, BufRead};

struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}

impl<R: BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: Vec::new(),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("falha ao ler entrada.");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("erro na leitura.");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

fn calcula_quadrante(x: f32, y: f32) -> String {
    if x == 0.0 && y == 0.0 {
        return "Origem".to_string();
    } else if y == 0.0 {
        return "Eixo X".to_string();
    } else if x == 0.0 {
        return "Eixo Y".to_string();
    } else if x > 0.0 {
        if y > 0.0 {
            return "Q1".to_string();
        } else {
            return "Q4".to_string();
        }
    } else {
        if y > 0.0 {
            return "Q2".to_string();
        } else {
            return "Q3".to_string();
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    let x: f32 = scanner.next();
    let y: f32 = scanner.next();

    let quadrante = calcula_quadrante(x, y);
    println!("{}", quadrante);
}
