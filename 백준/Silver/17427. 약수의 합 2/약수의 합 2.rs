use std::io::{self, Read};

fn calculate_g(n: usize) -> Vec<u64> {
    let mut f = vec![1; n + 1]; // 모든 수는 최소 1을 약수로 가짐
    let mut g = vec![0; n + 1]; // g(x)를 저장할 벡터

    // 에라토스테네스의 체와 유사한 방법으로 약수의 합을 계산
    for i in 2..=n {
        for j in (i..=n).step_by(i) {
            f[j] += i as u64;
        }
    }

    // g(x) 계산
    for i in 1..=n {
        g[i] = g[i - 1] + f[i];
    }

    g
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut numbers = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n = numbers.next().unwrap();
    let g = calculate_g(1_000_000); // g(x)를 미리 계산

    println!("{}", g[n]);
}
