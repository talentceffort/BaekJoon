use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let t: usize = input.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let w: usize = input.next().unwrap().parse().unwrap();
        let h: usize = input.next().unwrap().parse().unwrap();

        let mut board = Vec::with_capacity(h);
        let mut fire_map = vec![vec![-1; w]; h];
        let mut visited = vec![vec![false; w]; h];

        let mut fire = VecDeque::new();
        let mut sang = VecDeque::new();

        for _ in 0..h {
            let mut row = Vec::with_capacity(w);
            for c in input.next().unwrap().chars() {
                row.push(c);
            }
            board.push(row);
        }

        for x in 0..h {
            for y in 0..w {
                let c = board[x][y];
                if c == '@' {
                    visited[x][y] = true;
                    sang.push_back((x, y, 0));
                }
                if c == '*' {
                    fire.push_back((x, y, 0));
                    fire_map[x][y] = 0;
                }
            }
        }

        let directions: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

        while let Some((x, y, t)) = fire.pop_front() {
            for (dx, dy) in directions.iter() {
                let nx = (dx + x as isize) as usize;
                let ny = (dy + y as isize) as usize;
                if nx < h && ny < w {
                    if board[nx][ny] == '.' && fire_map[nx][ny] == -1 {
                        fire.push_back((nx, ny, t + 1));
                        fire_map[nx][ny] = t + 1;
                    }
                }
            }
        }

        let mut is_success = false;

        while let Some((x, y, t)) = sang.pop_front() {
            if x == 0 || x == h - 1 || y == 0 || y == w - 1 {
                println!("{}", t + 1);
                is_success = true;
                break;
            }
            for (dx, dy) in directions.iter() {
                let nx = (dx + x as isize) as usize;
                let ny = (dy + y as isize) as usize;
                if nx < h && ny < w {
                    if board[nx][ny] == '.'
                        && !visited[nx][ny]
                        && (fire_map[nx][ny] == -1 || fire_map[nx][ny] > t + 1)
                    {
                        sang.push_back((nx, ny, t + 1));
                        visited[nx][ny] = true;
                    }
                }
            }
        }

        if !is_success {
            println!("IMPOSSIBLE");
        }
    }
}
