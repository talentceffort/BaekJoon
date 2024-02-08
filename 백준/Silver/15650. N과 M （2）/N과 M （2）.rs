use std::fmt::Write;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();

    let n: usize = input.next().unwrap().parse().unwrap();
    let m: usize = input.next().unwrap().parse().unwrap();

    let mut array: Vec<usize> = vec![0; 10];
    let mut is_used = vec![false; 10];

    func(&mut array, &mut is_used, n, m, 0);
}

fn func(array: &mut Vec<usize>, is_used: &mut Vec<bool>, n: usize, m: usize, k: usize) {
    if m == k {
        let mut output = String::new();
        for i in 0..m {
            write!(output, "{} ", array[i]).unwrap();
        }
        writeln!(output).unwrap();
        print!("{}", output);
        return;
    }
    let mut st = 1; // 시작지점, k = 0일 때에는 st = 1
    if k != 0 {
        st = array[k - 1] + 1; // k != 0일 경우 st = arr[k - 1] + 1
    }
    for i in st..n + 1 {
        if !is_used[i] {
            array[k] = i;
            is_used[i] = true;
            func(array, is_used, n, m, k + 1);
            is_used[i] = false;
        }
    }
}
