use std::io::{self, Read};

fn check(paper: &Vec<Vec<i32>>, x: usize, y: usize, n: usize) -> bool {
    for i in x..x + n {
        for j in y..y + n {
            if paper[x][y] != paper[i][j] {
                return false;
            }
        }
    }
    true
}

fn solve(paper: &Vec<Vec<i32>>, cnt: &mut [i32; 3], x: usize, y: usize, z: usize) {
    if check(paper, x, y, z) {
        cnt[(paper[x][y] + 1) as usize] += 1;
        return;
    }
    let n = z / 3;
    for i in 0..3 {
        for j in 0..3 {
            // (x + i) * n 과 (y + j) * n 은 작은 종이의 시작 위치 계산
            solve(paper, cnt, x + i * n, y + j * n, n);
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    let n: usize = input.next().unwrap().parse().unwrap();

    let mut paper: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        let row: Vec<i32> = (0..n)
            .map(|_| input.next().unwrap().parse().unwrap())
            .collect();
        paper.push(row);
    }

    let mut cnt: [i32; 3] = [0; 3];
    solve(&paper, &mut cnt, 0, 0, n);
    for i in 0..3 {
        println!("{}", cnt[i]);
    }
}
