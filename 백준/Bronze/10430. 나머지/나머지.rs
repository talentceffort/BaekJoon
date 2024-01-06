use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let a = input.next().unwrap();
    let b = input.next().unwrap();
    let x = input.next().unwrap();

    println!("{}", (a + b) % x);
    println!("{}", ((a % x) + (b % x)) % x);
    println!("{}", (a * b) % x);
    println!("{}", ((a % x) * (b % x)) % x);
}
