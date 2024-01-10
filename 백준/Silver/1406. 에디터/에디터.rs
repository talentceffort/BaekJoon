use std::collections::VecDeque;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let mut left_stack: VecDeque<char> = input.next().unwrap().chars().collect();
    let mut right_stack: VecDeque<char> = VecDeque::new();

    let n = input.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..n {
        let command: Vec<char> = input.next().unwrap().chars().collect();
        match command[0] {
            'L' => {
                if let Some(c) = left_stack.pop_back() {
                    right_stack.push_back(c);
                }
            }
            'D' => {
                if let Some(c) = right_stack.pop_back() {
                    left_stack.push_back(c);
                }
            }
            'B' => {
                left_stack.pop_back();
            }
            'P' => {
                left_stack.push_back(command[2]);
            }
            _ => {}
        }
    }

    let result: String = left_stack
        .into_iter()
        .chain(right_stack.into_iter().rev())
        .collect();
    println!("{}", result);
}
