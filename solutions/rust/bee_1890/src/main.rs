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

fn solve<R: BufRead>(scanner: &mut Scanner<R>) {
    let numeros: i32 = scanner.next();
    let consoantes: i32 = scanner.next();
    let resultado = calcula_placas(numeros, consoantes);
    println!("{}", resultado);
}

fn calcula_placas(n: i32, c: i32) -> i32 {
    if n == 0 && c == 0 {
        return 0;
    }
    (26_i32.pow(n as u32) * 10_i32.pow(c as u32)) as i32
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock()); 

    let t: usize = scanner.next(); 

    for _ in 0..t {
        solve(&mut scanner);
    }
}
