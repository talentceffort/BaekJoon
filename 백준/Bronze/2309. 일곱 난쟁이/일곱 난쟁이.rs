use std::io;

fn main() {
    let mut heights = Vec::new();
    let mut input = String::new();
    for _ in 0..9 {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        heights.push(input.trim().parse::<i32>().unwrap());
    }

    let total_sum: i32 = heights.iter().sum();
    let mut found = false;

    for i in 0..9 {
        for j in i + 1..9 {
            if total_sum - heights[i] - heights[j] == 100 {
                heights.remove(j);
                heights.remove(i);
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }

    heights.sort();
    for height in heights {
        println!("{}", height);
    }
}