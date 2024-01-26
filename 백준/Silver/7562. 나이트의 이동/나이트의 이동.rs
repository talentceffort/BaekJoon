use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let t: usize = input.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let l: usize = input.next().unwrap().parse().unwrap();
        let mut board = vec![vec![false; l]; l];
        let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
        let start_x: usize = input.next().unwrap().parse().unwrap();
        let start_y: usize = input.next().unwrap().parse().unwrap();
        let end_x: usize = input.next().unwrap().parse().unwrap();
        let end_y: usize = input.next().unwrap().parse().unwrap();

        let directions = [
            (1, 2), (2, 1), (-1, 2), (2, -1), (-1, -2), (-2, -1), (1, -2), (-2, 1),
        ];

        queue.push_back((start_x, start_y, 0));
        board[start_y][start_x] = true;

        while let Some((x, y, count)) = queue.pop_front() {
            if x == end_x && y == end_y {
                println!("{}", count);
                break;
            }

            for (dx, dy) in directions.iter() {
                let nx = (x as isize + dx) as usize;
                let ny = (y as isize + dy) as usize;
                if nx < l && ny < l && !board[ny][nx] {
                    board[ny][nx] = true;
                    queue.push_back((nx, ny, count + 1));
                }
            }
        }
    }
}