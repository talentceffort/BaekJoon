use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    for line in input.lines() {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        if numbers[0] == "0" {
            break;
        }

        let numbers: Vec<usize> = numbers
            .iter()
            .skip(1) // 첫 번째 숫자(k)는 건너뜀
            .map(|&num| num.parse().unwrap())
            .collect();

        let mut output = String::new();
        let mut stack: Vec<usize> = Vec::with_capacity(15);
        let mut is_used = vec![false; 30];
        solve(&numbers, &mut is_used, 0, 0, &mut stack, &mut output);
        println!("{}", output)
    }
}

fn solve(
    numbers: &Vec<usize>,
    is_used: &mut Vec<bool>,
    start: usize,
    depth: usize,
    stack: &mut Vec<usize>,
    output: &mut String,
) {
    if depth == 6 {
        writer(output, stack, numbers);
        return;
    }

    // 중복을 허용하지 않기 위해 start로 시작 index 조정.
    for i in start..numbers.len() {
        if !is_used[i] {
            is_used[i] = true;
            stack.push(i);
            solve(numbers, is_used, i + 1, depth + 1, stack, output);
            is_used[i] = false;
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
