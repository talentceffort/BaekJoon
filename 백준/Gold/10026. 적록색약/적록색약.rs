use std::collections::VecDeque;
use std::io::{stdin, Read};
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let n: usize = input.next().unwrap().parse().unwrap();

    let mut board = Vec::with_capacity(n);
    let mut board_for_colorblind = Vec::with_capacity(n);
    let mut visited = vec![vec![false; n]; n];
    let mut visited_for_colorblind = vec![vec![false; n]; n];

    for _ in 0..n {
        let mut row = Vec::with_capacity(n);
        let mut row_for_colorblind = Vec::with_capacity(n);
        for c in input.next().unwrap().chars() {
            row.push(c);
            row_for_colorblind.push(if c == 'G' { 'R' } else { c });
        }
        board.push(row);
        board_for_colorblind.push(row_for_colorblind);
    }

    let mut count = 0;
    let mut count_for_colorblind = 0;

    for i in 0..n {
        for j in 0..n {
            if !visited[i][j] {
                bfs(n, i, j, &mut visited, &board);
                count += 1;
            }

            if !visited_for_colorblind[i][j] {
                bfs(n, i, j, &mut visited_for_colorblind, &board_for_colorblind);
                count_for_colorblind += 1;
            }
        }
    }
    println!("{} {}", count, count_for_colorblind);
}

fn bfs(n: usize, x: usize, y: usize, visited: &mut Vec<Vec<bool>>, board: &Vec<Vec<char>>) {
    let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];
    let mut queue = VecDeque::new();
    queue.push_back((x, y));
    visited[x][y] = true;
    let color = board[x][y];

    while let Some((qx, qy)) = queue.pop_front() {
        for (dx, dy) in directions {
            let nx_isize = qx as isize + dx;
            let ny_isize = qy as isize + dy;
            if nx_isize >= 0 && nx_isize < n as isize && ny_isize >= 0 && ny_isize < n as isize {
                let nx = nx_isize as usize;
                let ny = ny_isize as usize;
                if !visited[nx][ny] && board[nx][ny] == color {
                    visited[nx][ny] = true;
                    queue.push_back((nx, ny));
                }
            }
        }
    }
}
