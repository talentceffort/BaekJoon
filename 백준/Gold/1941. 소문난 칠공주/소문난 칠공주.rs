use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let board: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    println!("{}", find_princess_combinations(&board));
}

fn find_princess_combinations(board: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    let mut selected = vec![false; 25]; // 25명 중에서 7명을 선택하는 조합을 표현하는 배열

    // 백트래킹을 시작합니다.
    backtrack(&board, &mut selected, 0, 0, &mut count);
    count
}

fn backtrack(
    board: &Vec<Vec<char>>,
    selected: &mut Vec<bool>,
    idx: usize,
    depth: usize,
    count: &mut usize,
) {
    if depth == 7 {
        // 7명이 선택되었을 때, 조건을 확인합니다.
        if is_valid(board, selected) {
            *count += 1;
        }
        return;
    }
    if idx == 25 {
        return;
    }

    // 현재 위치를 선택하는 경우
    selected[idx] = true;
    backtrack(board, selected, idx + 1, depth + 1, count);

    // 현재 위치를 선택하지 않는 경우
    selected[idx] = false;
    backtrack(board, selected, idx + 1, depth, count);
}

fn is_valid(board: &Vec<Vec<char>>, selected: &Vec<bool>) -> bool {
    let mut s_count = 0;
    let mut start = None;
    let mut visited = vec![vec![false; 5]; 5];

    for i in 0..25 {
        if selected[i] {
            let x = i / 5;
            let y = i % 5;
            if board[x][y] == 'S' {
                s_count += 1;
            }
            if start.is_none() {
                start = Some((x, y));
            }
            visited[x][y] = true;
        }
    }

    // 이다솜파('S')가 4명 미만이면 조건을 만족하지 않습니다.
    if s_count < 4 {
        return false;
    }

    // 선택된 학생들이 서로 이어져 있는지 확인합니다.
    let mut seen = 0;
    let mut queue = VecDeque::new();
    if let Some(s) = start {
        queue.push_back(s);
        visited[s.0][s.1] = false; // 시작점을 방문 처리합니다.
        seen += 1; // 시작점도 방문한 위치이므로 +1
        while let Some((x, y)) = queue.pop_front() {
            for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < 5 && ny >= 0 && ny < 5 && visited[nx as usize][ny as usize] {
                    visited[nx as usize][ny as usize] = false; // 방문한 곳은 다시 방문하지 않도록 처리합니다.
                    queue.push_back((nx as usize, ny as usize));
                    seen += 1;
                }
            }
        }
    }

    // 모든 선택된 학생들이 서로 이어져 있어야 합니다.
    // BFS를 통해 방문한 위치의 수(`seen`)가 선택된 위치의 총 수(`selected_positions`)와 일치해야 합니다.
    seen == 7
}
