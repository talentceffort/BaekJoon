use std::io::{self, stdout, BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let m: usize = input.next().unwrap().parse().unwrap();

    let mut is_used = vec![false; 10];
    let mut nums: Vec<usize> = Vec::with_capacity(n);
    let mut output = String::new();
    let mut num = vec![0; n];
    let mut array: Vec<usize> = Vec::with_capacity(n);

    for _ in 0..n {
        array.push(input.next().unwrap().parse().unwrap());
    }
    for i in 0..n {
        num[i] = array[i];
    }

    num.sort();
    solve(0, n, m, &mut is_used, &mut nums, &num, &mut output);
    println!("{}", output);
}

fn solve(
    depth: usize,
    n: usize,
    m: usize,
    is_used: &mut Vec<bool>,
    nums: &mut Vec<usize>,
    num: &Vec<usize>,
    output: &mut String,
) {
    if depth == m {
        writer(output, nums);
        return;
    }

    for i in 0..n {
        if !is_used[i] {
            is_used[i] = true;
            nums.push(num[i]);
            solve(depth + 1, n, m, is_used, nums, num, output);
            nums.pop();
            is_used[i] = false;
        }
    }
}

fn writer(writer: &mut String, nums: &Vec<usize>) {
    for &num in nums.iter() {
        writer.push_str(&num.to_string());
        writer.push(' ');
    }
    writer.push_str("\n");
}
