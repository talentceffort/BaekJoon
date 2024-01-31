use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let a: i64 = input.next().unwrap().parse().unwrap();
    let b: i64 = input.next().unwrap().parse().unwrap();
    let c: i64 = input.next().unwrap().parse().unwrap();

    // 자연수 A를 B번 곱한 수를 알고 싶다. 단 구하려는 수가 매우 커질 수 있으므로 이를 C로 나눈 나머지를 구하는 프로그램을 작성하시오.
    let result = pow(a, b, c);
    println!("{}", result);
}

fn pow(a: i64, b: i64, m: i64) -> i64 {
    if b == 1 {
        return a % m;
    }
    let mut val = pow(a, b / 2, m);
    val = val * val % m;
    if b % 2 == 0 {
        return val;
    }
    return val * a % m;
}
