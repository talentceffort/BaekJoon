use std::io::{self, Read};
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut array = vec![0; 10];

    for x in numbers {
        if x == 6 || x == 9 {
            if array[6] > array[9] {
                array[9] += 1;
            } else {
                array[6] += 1;
            }
        } else {
            array[x as usize] += 1;
        }
    }
    println!("{:?}", array.iter().max().unwrap());
}
