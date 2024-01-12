use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let n: usize = input.next().unwrap().parse().unwrap();
    let mut good_word_count = 0;

    for _ in 0..n {
        let word: String = input.next().unwrap().to_string();
        let mut stack: Vec<char> = Vec::new();

        for ch in word.chars() {
            if stack.last() == Some(&ch) {
                stack.pop();
            } else {
                stack.push(ch);
            }
        }

        if stack.is_empty() {
            good_word_count += 1;
        }
    }

    println!("{}", good_word_count);
}