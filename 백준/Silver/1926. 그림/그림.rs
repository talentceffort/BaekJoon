use std::collections::VecDeque;
use std::io::{stdin, Read};

fn bfs(
    board: &Vec<Vec<usize>>,
    visited: &mut Vec<Vec<bool>>,
    x: usize,
    y: usize,
    n: usize,
    m: usize,
) -> usize {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut area = 1;

    visited[x][y] = true;
    queue.push_back((x, y));

    while let Some((cx, cy)) = queue.pop_front() {
        for &(dx, dy) in directions.iter() {
            let nx = cx as isize + dx;
            let ny = cy as isize + dy;
            if nx >= 0 && nx < n as isize && ny >= 0 && ny < m as isize {
                let nx = nx as usize;
                let ny = ny as usize;
                if board[nx][ny] == 1 && !visited[nx][ny] {
                    visited[nx][ny] = true;
                    queue.push_back((nx, ny));
                    area += 1;
                }
            }
        }
    }

    area
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let m: usize = input.next().unwrap().parse().unwrap();

    let mut board: Vec<Vec<usize>> = Vec::with_capacity(n);
    let mut visited: Vec<Vec<bool>> = vec![vec![false; m]; n];

    for _ in 0..n {
        let mut row: Vec<usize> = Vec::with_capacity(m);
        for _ in 0..m {
            let value: usize = input.next().unwrap().parse().unwrap();
            row.push(value);
        }
        board.push(row);
    }

    let mut number_of_pictures = 0;
    let mut max_area = 0;

    for i in 0..n {
        for j in 0..m {
            if board[i][j] == 1 && !visited[i][j] {
                number_of_pictures += 1;
                let area = bfs(&board, &mut visited, i, j, n, m);
                if area > max_area {
                    max_area = area;
                }
            }
        }
    }

    println!("{}", number_of_pictures);
    println!("{}", max_area);
}
