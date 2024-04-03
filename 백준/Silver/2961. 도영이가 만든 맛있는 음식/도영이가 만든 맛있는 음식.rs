use std::cmp::min;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let mut ingredients = Vec::new();

    for _ in 0..n {
        let s: usize = input.next().unwrap().parse().unwrap(); // 신맛
        let b: usize = input.next().unwrap().parse().unwrap(); // 쓴맛
        ingredients.push((s, b));
    }

    let result = recursive(&ingredients, 1, 0, 0);
    println!("{result}");
}

fn recursive(ingredients: &Vec<(usize, usize)>, sour: usize, bitter: usize, index: usize) -> usize {
    if index == ingredients.len() {
        if sour == 1 && bitter == 0 {
            // 아무 재료도 사용하지 않은 경우
            return usize::MAX;
        }
        return (sour as isize - bitter as isize).abs() as usize;
    }

    let include = recursive(
        ingredients,
        sour * ingredients[index].0,
        bitter + ingredients[index].1,
        index + 1,
    );

    let exclude = recursive(ingredients, sour, bitter, index + 1);

    min(include, exclude)
}
