use std::{
    collections::VecDeque,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let t: usize = input.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let mut left_stack: VecDeque<char> = VecDeque::new();
        let mut right_stack: VecDeque<char> = VecDeque::new();

        let input = input.next().unwrap();
        for c in input.chars() {
            match c {
                '<' => {
                    if let Some(x) = left_stack.pop_back() {
                        right_stack.push_front(x);
                    }
                }
                '>' => {
                    if let Some(x) = right_stack.pop_front() {
                        left_stack.push_back(x);
                    }
                }
                '-' => {
                    left_stack.pop_back();
                }
                _ => left_stack.push_back(c),
            }
        }

        let result: String = left_stack
            .into_iter()
            .chain(right_stack.into_iter())
            .collect();
        println!("{}", result);
    }
}
