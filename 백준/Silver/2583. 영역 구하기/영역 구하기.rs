use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let m: usize = input.next().unwrap().parse().unwrap();
    let n: usize = input.next().unwrap().parse().unwrap();
    let k: usize = input.next().unwrap().parse().unwrap();

    let mut board = vec![vec![0; n]; m];

    for _ in 0..k {
        let start_y: usize = input.next().unwrap().parse().unwrap();
        let start_x: usize = input.next().unwrap().parse().unwrap();

        let end_y: usize = input.next().unwrap().parse().unwrap();
        let end_x: usize = input.next().unwrap().parse().unwrap();

        for i in start_x..end_x {
            for j in start_y..end_y {
                board[i][j] = 1;
            }
        }
    }

    let directions: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut areas = Vec::new();

    for x in 0..m {
        for y in 0..n {
            if board[x][y] == 0 {
                let mut queue = VecDeque::new();
                queue.push_back((x, y));
                board[x][y] = 1;
                let mut area = 0;

                while let Some((x, y)) = queue.pop_front() {
                    area += 1;
                    for (dx, dy) in directions.iter() {
                        let nx = (x as isize + dx) as usize;
                        let ny = (y as isize + dy) as usize;
                        if nx < m && ny < n && board[nx][ny] == 0 {
                            queue.push_back((nx, ny));
                            board[nx][ny] = 1;
                        }
                    }
                }
                areas.push(area);
            }
        }
    }
    areas.sort();
    println!("{}", areas.len());
    for area in areas {
        print!("{} ", area);
    }
}
