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

    fn next_vec(&mut self, n: usize) -> Vec<i32> {
        (0..n).map(|_| self.next()).collect()
    }
}

fn conta_multiplos(valores: &[i32]) {
    let mut count2 = 0;
    let mut count3 = 0;
    let mut count4 = 0;
    let mut count5 = 0;

    for &valor in valores {
        if valor % 2 == 0 {
            count2 += 1;
        }
        if valor % 3 == 0 {
            count3 += 1;
        }
        if valor % 4 == 0 {
            count4 += 1;
        }
        if valor % 5 == 0 {
            count5 += 1;
        }
    }

    println!("{} Multiplo(s) de 2", count2);
    println!("{} Multiplo(s) de 3", count3);
    println!("{} Multiplo(s) de 4", count4);
    println!("{} Multiplo(s) de 5", count5);
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    let casos: usize = scanner.next();
    let valores = scanner.next_vec(casos);

    conta_multiplos(&valores);
}
