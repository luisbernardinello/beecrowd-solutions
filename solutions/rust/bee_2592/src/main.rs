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
            self.reader.read_line(&mut input).expect("erro ao ler linha.");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }

    fn next_vec(&mut self, n: usize) -> Vec<usize> {
        (0..n).map(|_| self.next()).collect()
    }
}

fn main() {
    let stdin = io::stdin();
    let mut scanner = Scanner::new(stdin.lock());

    loop {
        let casos: usize = scanner.next();
        
        if casos == 0 {
            break;
        }

        let ordem_correta: Vec<usize> = (1..=casos).collect();
        let mut tentativas = 0;

        loop {
            tentativas += 1;
            let valores = scanner.next_vec(casos);

            if valores == ordem_correta {
                break;
            }
        }

        println!("{}", tentativas);
    }
}
