use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<i32>);

    let mut input = input.next().unwrap();

    let mut count = 0;

    while input >= 0 {
        if input % 5 == 0 {
            count += input / 5;
            println!("{}", count);
            return;
        }
        
        input -= 3;
        count += 1;
    }

    println!("{}", -1);
}