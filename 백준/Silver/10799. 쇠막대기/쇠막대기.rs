use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut input = input.lines();

    let s = input.next().unwrap();
    let mut pieces = 0;
    let mut prev_char = ' ';
    let mut stack: Vec<char> = Vec::new();

    for c in s.chars() {
        match c {
            '(' => {
                stack.push('(');
            }
            ')' => {
                stack.pop();
                if prev_char == '(' {
                    // 레이저인 경우
                    pieces += stack.len();
                } else {
                    // 쇠막대기의 끝인 경우
                    pieces += 1;
                }
            }
            _ => unreachable!(),
        }
        prev_char = c;
    }
    println!("{}", pieces);
}
