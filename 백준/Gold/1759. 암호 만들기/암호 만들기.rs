use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut input = input.split_whitespace();

    let l: usize = input.next().unwrap().parse().unwrap();
    let c: usize = input.next().unwrap().parse().unwrap();

    let mut chars: Vec<char> = Vec::with_capacity(20);

    for _ in 0..c {
        chars.push(input.next().unwrap().parse::<char>().unwrap());
    }

    chars.sort();

    let mut output = String::new();
    let mut stack: Vec<usize> = Vec::with_capacity(20);
    solve(l, &chars, 0, 0, &mut stack, &mut output);
    println!("{}", output)
}

fn vowel_checker(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

fn solve(
    l: usize,
    chars: &Vec<char>,
    start: usize,
    depth: usize,
    stack: &mut Vec<usize>,
    output: &mut String,
) {
    if depth == l {
        // 최소 한 개의 모음(a, e, i, o, u)과 최소 두 개의 자음으로 구성 되어 있는지 판단.
        let mut consonant = 0; // 자음
        let mut vowel = 0; // 모음

        for i in stack.iter() {
            if vowel_checker(chars[*i]) {
                vowel += 1;
            } else {
                consonant += 1;
            }
        }
        if vowel >= 1 && consonant >= 2 {
            writer(output, stack, chars);
        }
        return;
    }

    for i in start..chars.len() {
        stack.push(i);
        solve(l, chars, i + 1, depth + 1, stack, output);
        stack.pop();
    }
}

fn writer(writer: &mut String, stack: &Vec<usize>, chars: &Vec<char>) {
    for &n in stack.iter() {
        writer.push_str(&chars[n].to_string());
    }
    writer.push_str("\n");
}
