use std::fmt::Write;
use std::io::{self, Read};

const MAX: usize = 1024 * 3 + 2;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();

    let mut board: Vec<Vec<char>> = vec![vec![' '; n * 2 - 1]; MAX];
    let mut output = String::new();

    func(&mut board, n, 0, n - 1);

    for i in 0..n {
        for j in 0..(n * 2 - 1) {
            write!(output, "{}", board[i][j]).unwrap();
        }
        writeln!(output).unwrap();
    }
    println!("{}", output);
}

fn draw_star(board: &mut Vec<Vec<char>>, x: usize, y: usize) {
    board[x][y] = '*';
    board[x + 1][y - 1] = '*';
    board[x + 1][y + 1] = '*';

    for i in (y - 2)..=(y + 2) {
        board[x + 2][i] = '*';
    }
}

fn func(board: &mut Vec<Vec<char>>, s: usize, x: usize, y: usize) {
    if s == 3 {
        // println!("x: {}", x);
        // println!("y: {}", y);
        draw_star(board, x, y);
        return;
    }

    let ns = s / 2;
    // println!("x + ns: {}", x + ns);
    // println!("y - ns : {}", y - ns);
    // println!("y + ns: {}", y + ns);
    // println!("ns: {}", ns);
    func(board, ns, x, y); // 맨 위 삼각형
    func(board, ns, x + ns, y - ns); // 왼쪽아래 삼각형
    func(board, ns, x + ns, y + ns); // 오른쪽 아래 삼각형
}
