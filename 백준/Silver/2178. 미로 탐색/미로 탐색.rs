use std::collections::VecDeque;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let array: Vec<usize> = input
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let n: usize = array[0];
    let m: usize = array[1];

    let mut board: Vec<Vec<usize>> = Vec::with_capacity(n);
    let mut distance: Vec<Vec<isize>> = vec![vec![-1; m]; n];

    for _ in 0..n {
        let line = input.next().unwrap();
        let row: Vec<usize> = line
            .chars()
            .map(|x| x.to_digit(10).unwrap() as usize)
            .collect();
        board.push(row);
    }

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    distance[0][0] = 1;
    queue.push_back((0, 0));

    while let Some((cx, cy)) = queue.pop_front() {
        for &(dx, dy) in directions.iter() {
            let nx = cx as isize + dx;
            let ny = cy as isize + dy;
            if nx >= 0 && nx < n as isize && ny >= 0 && ny < m as isize {
                let nx = nx as usize;
                let ny = ny as usize;
                if board[nx][ny] == 1 && distance[nx][ny] == -1 {
                    distance[nx][ny] = distance[cx][cy] + 1;
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    println!("{:?}", distance[n - 1][m - 1]);
}
