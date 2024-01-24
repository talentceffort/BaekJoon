use std::collections::VecDeque;
use std::io::{stdin, Read};
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let t: usize = input.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let m: usize = input.next().unwrap().parse().unwrap();
        let n: usize = input.next().unwrap().parse().unwrap();
        let k: usize = input.next().unwrap().parse().unwrap();

        let mut board = vec![vec![0; m]; n];
        let mut visited = vec![vec![false; m]; n];

        for _ in 0..k {
            let x: usize = input.next().unwrap().parse().unwrap();
            let y: usize = input.next().unwrap().parse().unwrap();
            board[y][x] = 1;
        }

        let mut worms = 0;
        for y in 0..n {
            for x in 0..m {
                if board[y][x] == 1 && !visited[y][x] {
                    bfs(&mut board, &mut visited, x, y, m, n);
                    worms += 1;
                }
            }
        }
        println!("{worms}")
    }
}

fn bfs(
    board: &mut Vec<Vec<i32>>,
    visited: &mut Vec<Vec<bool>>,
    x: usize,
    y: usize,
    m: usize,
    n: usize,
) {
    let mut queue = VecDeque::new();
    let direction: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    queue.push_back((y, x));
    visited[y][x] = true;

    while let Some((y, x)) = queue.pop_front() {
        for &(dy, dx) in direction.iter() {
            let ny = y as isize + dy;
            let nx = x as isize + dx;
            if ny >= 0 && ny < n as isize && nx >= 0 && nx < m as isize {
                let ny = ny as usize;
                let nx = nx as usize;
                if !visited[ny][nx] && board[ny][nx] == 1 {
                    visited[ny][nx] = true;
                    queue.push_back((ny, nx));
                }
            }
        }
    }
}
