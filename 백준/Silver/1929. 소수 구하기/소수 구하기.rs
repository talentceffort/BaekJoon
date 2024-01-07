use std::fmt::Write;
use std::io::{self, Read};
fn sieve_of_eratosthenes(n: usize) -> Vec<bool> {
    let mut prime = vec![true; n + 1];
    let mut array: Vec<usize> = Vec::with_capacity(n + 1);
    prime[0] = false;
    prime[1] = false;

    for p in 2..=n {
        if prime[p] {
            let mut multiple = p * p;
            array.push(p);
            while multiple <= n {
                prime[multiple] = false;
                multiple += p;
            }
        }
    }

    prime
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut numbers = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let m = numbers.next().unwrap();
    let n = numbers.next().unwrap();

    let prime = sieve_of_eratosthenes(n);

    let mut output = String::new();

    for (index, &is_prime) in prime.iter().enumerate() {
        if is_prime && index >= m && index <= n {
            // println!("{}", index);
            writeln!(output, "{}", index).unwrap();
        }
    }

    println!("{output}");
}