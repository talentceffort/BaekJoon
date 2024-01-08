use std::io::{self, Read};
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<usize> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut array = vec![0; 10];

    for x in numbers {
        if x == 6 || x == 9 {
            array[6] += 1;
        } else {
            array[x] += 1;
        }
    }

    array[6] = (array[6] + 1) / 2;
    println!("{}", array.iter().max().unwrap());
}
