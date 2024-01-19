use std::collections::VecDeque;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let r: isize = input.next().unwrap().parse().unwrap();
    let c: isize = input.next().unwrap().parse().unwrap();

    let mut map: Vec<Vec<char>> = Vec::new();

    let mut fire_time: Vec<Vec<isize>> = vec![vec![-1; c as usize]; r as usize];
    let mut fire: VecDeque<(isize, isize)> = VecDeque::new(); // (x, y, time)
    let mut jihoon: VecDeque<(isize, isize, isize)> = VecDeque::new(); // (x, y, time)

    for i in 0..r {
        let row: Vec<char> = input.next().unwrap().chars().collect();
        for (j, &item) in row.iter().enumerate() {
            match item {
                'J' => jihoon.push_back((i, j as isize, 0)),
                'F' => {
                    fire.push_back((i, j as isize));
                    fire_time[i as usize][j as usize] = 0;
                }
                _ => {}
            }
        }
        map.push(row);
    }

    let directions: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    while let Some((x, y)) = fire.pop_front() {
        for &(dx, dy) in &directions {
            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0
                && nx < r
                && ny >= 0
                && ny < c
                && map[nx as usize][ny as usize] != '#'
                && fire_time[nx as usize][ny as usize] == -1
            {
                fire_time[nx as usize][ny as usize] = fire_time[x as usize][y as usize] + 1;
                fire.push_back((nx, ny));
            }
        }
    }

    let mut escaped = false;
    let mut answer = 0;

    while let Some((x, y, time)) = jihoon.pop_front() {
        if x == 0 || y == 0 || x == r - 1 || y == c - 1 {
            escaped = true;
            answer = time + 1;
            break;
        }

        for &(dx, dy) in &directions {
            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0 && nx < r && ny >= 0 && ny < c && map[nx as usize][ny as usize] == '.' {
                if time + 1 < fire_time[nx as usize][ny as usize]
                    || fire_time[nx as usize][ny as usize] == -1
                {
                    map[nx as usize][ny as usize] = 'J';
                    jihoon.push_back((nx, ny, time + 1));
                }
            }
        }
    }

    if escaped {
        println!("{}", answer);
    } else {
        println!("IMPOSSIBLE");
    }
}
