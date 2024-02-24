use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let m: usize = input.next().unwrap().parse().unwrap();

    let mut stack: Vec<usize> = Vec::with_capacity(n);
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
    solve(0, 0, n, m, &mut stack, &num, &mut output);
    println!("{}", output);
}

fn solve(
    depth: usize,
    start: usize,
    n: usize,
    m: usize,
    stack: &mut Vec<usize>,
    num: &Vec<usize>,
    output: &mut String,
) {
    if depth == m {
        writer(output, stack, num);
        return;
    }

    let mut last_used = 0;

    for i in start..n {
        if last_used != num[i] {
            last_used = num[i];
            stack.push(i);
            solve(depth + 1, i + 1, n, m, stack, num, output);
            stack.pop();
        }
    }
}

fn writer(writer: &mut String, stack: &Vec<usize>, num: &Vec<usize>) {
    for &n in stack.iter() {
        writer.push_str(&num[n].to_string());
        writer.push(' ');
    }
    writer.push_str("\n");
}
