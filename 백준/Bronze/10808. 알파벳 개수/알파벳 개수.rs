use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let word = input.trim();

    let mut counts = [0; 26];
    for c in word.chars() {
        counts[(c as usize) - ('a' as usize)] += 1;
    }

    for count in &counts {
        print!("{} ", count);
    }
}
