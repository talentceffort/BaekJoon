use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let s: i32 = input.next().unwrap().parse().unwrap();

    let mut nums: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        nums.push(input.next().unwrap().parse::<i32>().unwrap());
    }

    let mut count = 0;
    find_subsequences(&nums, s, 0, 0, &mut count);

    if s == 0 {
        count -= 1;
    }
    println!("{}", count);
}

fn find_subsequences(
    nums: &Vec<i32>,
    target: i32,
    index: usize,
    current_sum: i32,
    count: &mut i32,
) {
    if index == nums.len() {
        if current_sum == target {
            *count += 1;
        }
        return;
    }

    // 현재 인덱스의 수를 포함하는 경우
    find_subsequences(nums, target, index + 1, current_sum + nums[index], count);
    // 현재 인덱스의 수를 포함하지 않는 경우
    find_subsequences(nums, target, index + 1, current_sum, count);
}
