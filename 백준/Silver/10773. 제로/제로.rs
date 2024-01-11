use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let t: usize = input.next().unwrap().parse().unwrap();
    let mut stack: Vec<i32> = Vec::with_capacity(t);

    for _ in 0..t {
        let num: i32 = input.next().unwrap().parse().unwrap();

        if num == 0 {
            stack.pop().unwrap();
        } else {
            stack.push(num)
        }
    }

    println!("{}", stack.iter().sum::<i32>());
}