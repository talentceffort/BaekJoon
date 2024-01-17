use std::collections::VecDeque;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let m: usize = input.next().unwrap().parse().unwrap(); 
    let n: usize = input.next().unwrap().parse().unwrap();

    let mut board: Vec<Vec<i32>> = Vec::with_capacity(n);
    let mut queue: VecDeque<(usize, usize, i32)> = VecDeque::new(); 

    for i in 0..n {
        let mut row: Vec<i32> = Vec::with_capacity(m);
        for j in 0..m {
            let num: i32 = input.next().unwrap().parse().unwrap();
            if num == 1 {
                queue.push_back((i, j, 0)); // 익은 토마토 위치와 시작일(0)을 큐에 추가
            }
            row.push(num);
        }
        board.push(row);
    }

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut max_days = 0;

    while let Some((x, y, days)) = queue.pop_front() {
        for &(dx, dy) in directions.iter() {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < n as isize && ny >= 0 && ny < m as isize {
                let nx = nx as usize;
                let ny = ny as usize;
                if board[nx][ny] == 0 {
                    board[nx][ny] = 1; // 토마토를 익게 함
                    queue.push_back((nx, ny, days + 1)); // 다음 날짜로 큐에 추가
                    max_days = days + 1; // 최대 일수 갱신
                }
            }
        }
    }

    // 모든 토마토가 익었는지 확인
    let all_ripe = board.iter().all(|row| row.iter().all(|&tomato| tomato != 0));

    if all_ripe {
        println!("{}", max_days);
    } else {
        println!("-1");
    }
}