use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let m: usize = input.next().unwrap().parse().unwrap();

    let mut is_used = vec![false; 10];
    let mut nums: Vec<usize> = Vec::with_capacity(n);
    let mut output = String::new();

    solve(0, n, m, &mut is_used, &mut nums, &mut output);

    println!("{}", output);
}

fn solve(
    depth: usize,
    n: usize,
    m: usize,
    is_used: &mut Vec<bool>,
    nums: &mut Vec<usize>,
    output: &mut String,
) {
    if depth == m {
        writer(output, nums);
        return;
    }

   
    for i in 1..n + 1 {
        if !is_used[i] {
            nums.push(i);
            solve(depth + 1, n, m, is_used, nums, output);
            nums.pop();
        }
    }
}

fn writer(writer: &mut String, nums: &Vec<usize>) {
    for num in nums {
        writer.push(std::char::from_digit(*num as u32, 10).unwrap());
        writer.push_str(" ");
    }
    writer.push_str("\n");
}
