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

static VISITED: i32 = -1;

fn search_area_size(mosaic: &mut Vec<Vec<i32>>, i: usize, j: usize, m: usize, n: usize, color: i32) -> i32 {
    if i >= m || j >= n || mosaic[i][j] != color {
        return 0;
    }

    mosaic[i][j] = VISITED;
    let mut size = 1;

    if i > 0 {
        size += search_area_size(mosaic, i - 1, j, m, n, color); //cima
    }
    if i < m - 1 {
        size += search_area_size(mosaic, i + 1, j, m, n, color); //baixo
    }
    if j > 0 {
        size += search_area_size(mosaic, i, j - 1, m, n, color);//esquerda
    }
    if j < n - 1 {
        size += search_area_size(mosaic, i, j + 1, m, n, color);//direita
    }

    size
}

fn solve() {
    let dimensions: Vec<usize> = read_vec!();
    let m = dimensions[0];
    let n = dimensions[1];

    let mut mosaic: Vec<Vec<i32>> = Vec::with_capacity(m);
    for _ in 0..m {
        mosaic.push(read_vec!());
    }

    let mut smallest_area_size = i32::MAX;

    for i in 0..m {
        for j in 0..n {
            if mosaic[i][j] != VISITED {
                let current_color = mosaic[i][j];
                let area_size = search_area_size(&mut mosaic, i, j, m, n, current_color);
                
                if smallest_area_size == -1 || area_size < smallest_area_size {
                    smallest_area_size = area_size;

                    if smallest_area_size == 1 {
                        println!("{}", smallest_area_size);
                        return;
                    }
                }
            }
        }
    }

    println!("{}", smallest_area_size);
}

// ========================= main ============================

fn main() {
    solve();
}
