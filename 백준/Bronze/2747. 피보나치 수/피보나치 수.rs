use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();

    let mut memo = vec![0; 50];
    memo[0] = 0;
    memo[1] = 1;

    func(&mut memo, n);

    println!("{}", memo[n]);
}

fn func(memo: &mut Vec<usize>, n: usize) -> usize {
    if n < 2 {
        return memo[n];
    }

    if memo[n] != 0 {
        return memo[n];
    }

    let result = func(memo, n - 1) + func(memo, n - 2);
    memo[n] = result;

    return result;
}
