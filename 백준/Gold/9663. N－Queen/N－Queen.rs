use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let mut count = 0;

    let mut is_used1 = vec![false; n]; // 각 열에 퀸이 있는지 여부
    let mut is_used2 = vec![false; 2 * n - 1]; // 주 대각선에 퀸이 있는지 여부
    let mut is_used3 = vec![false; 2 * n - 1]; // 부 대각선에 퀸이 있는지 여부

    func(
        &mut count,
        n,
        0,
        &mut is_used1,
        &mut is_used2,
        &mut is_used3,
    );

    println!("{}", count);
}

fn func(
    count: &mut usize,
    n: usize,
    cur: usize,
    is_used1: &mut Vec<bool>,
    is_used2: &mut Vec<bool>,
    is_used3: &mut Vec<bool>,
) {
    if cur == n {
        *count += 1;
        return;
    }

    for i in 0..n {
        if !is_used1[i] && !is_used2[cur + i] && !is_used3[cur + n - 1 - i] {
            is_used1[i] = true;
            is_used2[cur + i] = true;
            is_used3[cur + n - 1 - i] = true;

            func(count, n, cur + 1, is_used1, is_used2, is_used3);

            is_used1[i] = false;
            is_used2[cur + i] = false;
            is_used3[cur + n - 1 - i] = false;
        }
    }
}
