use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let s = input.next().unwrap();
    let mut stack: Vec<char> = Vec::new();
    let mut value_stack: Vec<i32> = Vec::new();
    let mut total_value = 0;
    let mut valid = true;

    for c in s.chars() {
        match c {
            '(' | '[' => {
                stack.push(c);
                value_stack.push(0);
            }
            ')' => {
                if stack.pop() != Some('(') {
                    valid = false;
                    break;
                }
                let value = value_stack.pop().unwrap();
                let add_value = if value == 0 { 2 } else { 2 * value };
                
                if let Some(last) = value_stack.last_mut() {
                    *last += add_value;
                } else {
                    total_value += add_value;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    valid = false;
                    break;
                }
                let value = value_stack.pop().unwrap();
                let add_value = if value == 0 { 3 } else { 3 * value };
                if let Some(last) = value_stack.last_mut() {
                    *last += add_value;
                } else {
                    total_value += add_value;
                }
            }
            _ => {}
        }
    }

    if valid && stack.is_empty() {
        println!("{}", total_value);
    } else {
        println!("0");
    }
}
