use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let t: usize = input.next().unwrap().parse().unwrap();
    let mut stack: Vec<i32> = Vec::with_capacity(t);

    for _ in 0..t {
        let parts: Vec<&str> = input.next().unwrap().split_whitespace().collect();

        match parts[0] {
            "push" => {
                let num: i32 = parts[1].parse().unwrap();
                stack.push(num);
            }
            "pop" => {
                if let Some(top) = stack.pop() {
                    println!("{}", top);
                } else {
                    println!("-1");
                }
            }
            "size" => {
                println!("{}", stack.len());
            }
            "empty" => {
                println!("{}", if stack.is_empty() { 1 } else { 0 });
            }
            "top" => {
                if let Some(&top) = stack.last() {
                    println!("{}", top);
                } else {
                    println!("-1");
                }
            }
            _ => unreachable!(),
        }
    }
}
