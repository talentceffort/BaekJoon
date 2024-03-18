use std::io::{self, Read};

#[derive(Debug, Clone, Copy)]
struct Egg {
    durability: i32,
    weight: i32,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();

    let mut eggs = Vec::new();

    for _ in 0..n {
        let durability = input.next().unwrap().parse().unwrap();
        let weight = input.next().unwrap().parse().unwrap();
        eggs.push(Egg { durability, weight })
    }
    println!("{}", max_broken_eggs(&mut eggs, 0));
}

fn max_broken_eggs(eggs: &mut [Egg], current: usize) -> usize {
    if current == eggs.len() {
        return eggs.iter().filter(|egg| egg.durability <= 0).count();
    }

    if eggs[current].durability <= 0 {
        return max_broken_eggs(eggs, current + 1);
    }

    let mut max_broken = 0;
    let mut is_hit = false;

    for i in 0..eggs.len() {
        if i != current && eggs[i].durability > 0 {
            is_hit = true;

            eggs[current].durability -= eggs[i].weight;
            eggs[i].durability -= eggs[current].weight;

            max_broken = max_broken.max(max_broken_eggs(eggs, current + 1));

            eggs[current].durability += eggs[i].weight;
            eggs[i].durability += eggs[current].weight;
        }
    }

    if !is_hit {
        max_broken = max_broken.max(max_broken_eggs(eggs, current + 1));
    }

    max_broken
}
