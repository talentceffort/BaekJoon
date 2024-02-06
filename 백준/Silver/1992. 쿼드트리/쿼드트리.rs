use std::fmt::Write;
use std::io::{self, Read};

fn check(paper: &Vec<Vec<i32>>, x: usize, y: usize, n: usize) -> bool {
    for i in x..x + n {
        for j in y..y + n {
            if paper[x][y] != paper[i][j] {
                return false;
            }
        }
    }
    true
}

fn solve(video: &Vec<Vec<i32>>, output: &mut String, x: usize, y: usize, z: usize) {
    if check(video, x, y, z) {
        write!(output, "{}", video[x][y]).unwrap();
        return;
    }
    write!(output, "(").unwrap();
    let n = z / 2;
    for i in 0..2 {
        for j in 0..2 {
            solve(video, output, x + i * n, y + j * n, n);
        }
    }
    write!(output, ")").unwrap();
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    let n: usize = input.next().unwrap().parse().unwrap();

    let mut video: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let mut row = Vec::with_capacity(n);
        for c in input.next().unwrap().chars() {
            row.push(c.to_digit(10).unwrap() as i32);
        }
        video.push(row);
    }

    let mut output = String::new();
    solve(&video, &mut output, 0, 0, n);

    println!("{}", output);
}
