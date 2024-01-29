use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let n: usize = input.next().unwrap().parse().unwrap();

    let mut map = Vec::with_capacity(n);
    let mut visited = vec![vec![false; n]; n];

    for _ in 0..n {
        let mut row = Vec::with_capacity(n);
        for c in input.next().unwrap().chars() {
            let c = c.to_digit(10).unwrap() as usize;
            row.push(c);
        }
        map.push(row);
    }

    let mut areas: Vec<i32> = Vec::new();

    for x in 0..n {
        for y in 0..n {
            if map[x][y] == 1 && visited[x][y] == false {
                let area = bfs(x, y, n, &mut visited, map.clone());
                areas.push(area);
            }
        }
    }
    areas.sort();
    println!("{}", areas.len());
    for area in areas {
        println!("{}", area);
    }
}

fn bfs(x: usize, y: usize, n: usize, visited: &mut Vec<Vec<bool>>, map: Vec<Vec<usize>>) -> i32 {
    let directions: [(isize, isize); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];
    let mut queue = VecDeque::new();
    queue.push_back((x, y));
    visited[x][y] = true;

    let mut area = 1; // 첫 시작점

    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in directions.iter() {
            let nx = (dx + x as isize) as usize;
            let ny = (dy + y as isize) as usize;

            if nx < n && ny < n {
                if !visited[nx][ny] && map[nx][ny] == 1 {
                    queue.push_back((nx, ny));
                    visited[nx][ny] = true;
                    area += 1;
                }
            }
        }
    }
    return area;
}
