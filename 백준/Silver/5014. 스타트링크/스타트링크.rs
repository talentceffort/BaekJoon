use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let f: usize = input.next().unwrap().parse().unwrap(); // 빌딩 층수
    let s: usize = input.next().unwrap().parse().unwrap(); // 현재 층
    let g: usize = input.next().unwrap().parse().unwrap(); // 스타트 링크 위치
    let u: usize = input.next().unwrap().parse().unwrap(); // 위로 u
    let d: i32 = input.next().unwrap().parse().unwrap(); // 아래로 d

    let moves: [i32; 2] = [u as i32, -d];
    let mut visited = vec![false; 1000001];
    let mut queue = VecDeque::new();
    queue.push_back((s, 0));
    visited[s] = true;

    while let Some((x, t)) = queue.pop_front() {
        if x == g {
            return println!("{}", t);
        }
        for &dx in moves.iter() {
            let nx = (dx + x as i32) as usize;
            if nx >= 1 && nx <= f && !visited[nx] {
                visited[nx] = true;
                queue.push_back((nx, t + 1));
            }
        }
    }
    println!("use the stairs");
}
