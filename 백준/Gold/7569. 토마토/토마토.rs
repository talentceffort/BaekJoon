use std::collections::VecDeque;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let m: usize = input.next().unwrap().parse().unwrap();
    let n: usize = input.next().unwrap().parse().unwrap();
    let h: usize = input.next().unwrap().parse().unwrap();

    let mut tomatoes: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; m]; n]; h];
    let mut queue: VecDeque<(usize, usize, usize, i32)> = VecDeque::new(); // (x, y, z, day)

    for z in 0..h {
        for y in 0..n {
            for x in 0..m {
                let tomato_status: i32 = input.next().unwrap().parse().unwrap();
                tomatoes[z][y][x] = tomato_status;
                if tomato_status == 1 {
                    queue.push_back((x, y, z, 0)); // 익은 토마토 위치와 시작일(0)을 큐에 추가
                }
            }
        }
    }


    let directions = [
        (0, 0, 1),
        (0, 0, -1),
        (0, 1, 0),
        (0, -1, 0),
        (1, 0, 0),
        (-1, 0, 0),
    ];
    let mut max_days = 0;

    while let Some((x, y, z, days)) = queue.pop_front() {
        for &(dx, dy, dz) in directions.iter() {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            let nz = z as isize + dz;

            if nx >= 0
                && nx < m as isize
                && ny >= 0
                && ny < n as isize
                && nz >= 0
                && nz < h as isize
            {
                let nx = nx as usize;
                let ny = ny as usize;
                let nz = nz as usize;
                if tomatoes[nz][ny][nx] == 0 {
                    tomatoes[nz][ny][nx] = 1; // 토마토를 익게 함
                    queue.push_back((nx, ny, nz, days + 1)); // 다음 날짜로 큐에 추가
                    max_days = days + 1; // 최대 일수 갱신
                }
            }
        }
    }

    // 모든 토마토가 익었는지 확인
    let all_ripe = tomatoes.iter().all(|layer| {
        layer
            .iter()
            .all(|row| row.iter().all(|&tomato| tomato != 0))
    });

    if all_ripe {
        println!("{}", max_days);
    } else {
        println!("-1");
    }
}
