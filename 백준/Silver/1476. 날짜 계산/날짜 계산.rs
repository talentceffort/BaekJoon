use std::io::{self, Read};
fn main() {
    // ------------------------------------------------------------------------------------------
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let numbers: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    let (e, s, m) = (numbers[0], numbers[1], numbers[2]);

    let (mut year_e, mut year_s, mut year_m, mut year) = (1, 1, 1, 1);

    while year_e != e || year_s != s || year_m != m {
        year_e = if year_e == 15 { 1 } else { year_e + 1 };
        year_s = if year_s == 28 { 1 } else { year_s + 1 };
        year_m = if year_m == 19 { 1 } else { year_m + 1 };
        year += 1;
    }

    println!("{}", year);

}