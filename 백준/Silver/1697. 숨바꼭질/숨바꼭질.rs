use std::collections::VecDeque;
use std::io::{stdin, Read};
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let k: usize = input.next().unwrap().parse().unwrap();

    let mut visited = vec![false; 100001];
    let mut queue = VecDeque::new();

    queue.push_back((n, 0));

    while let Some((current, time)) = queue.pop_front() {
        if current == k {
            return println!("{time}");
        }
        visited[current] = true;
        // 걷기
        if current > 0 && !visited[current - 1] {
            queue.push_back((current - 1, time + 1));
        }
        if current < 100000 && !visited[current + 1] {
            queue.push_back((current + 1, time + 1));
        }
        // 순간이동
        if current * 2 <= 100000 && !visited[current * 2] {
            queue.push_back((current * 2, time + 1));
        }
    }
}
