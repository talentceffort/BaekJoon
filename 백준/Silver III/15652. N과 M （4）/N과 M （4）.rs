use std::io::{self, stdout, BufWriter, Read, Write};

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
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout);
        for i in 0..m {
            write!(writer, "{} ", array[i]).unwrap();
        }
        writeln!(writer).unwrap();
        return;
    }

    let mut st = 1; // 시작지점, k = 0일 때에는 st = 1
    if k != 0 {
        st = array[k - 1]; // k != 0일 경우 st = arr[k-1]
    }

    for i in st..n + 1 {
        array[k] = i;
        func(array, is_used, n, m, k + 1);
    }
}
