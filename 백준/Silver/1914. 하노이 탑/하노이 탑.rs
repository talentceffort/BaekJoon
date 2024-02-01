use std::io::Write;
use std::io::{self, BufWriter, Read};
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace();
    let n: usize = input.next().unwrap().parse().unwrap();
    let count: i128 = (1 << n) - 1;

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    writeln!(writer, "{count}").unwrap();
    if n <= 20 {
        hanoi(1, 3, n, &mut writer);
    }
}

fn hanoi(a: usize, b: usize, n: usize, writer: &mut impl std::io::Write) {
    if n == 1 {
        writeln!(writer, "{a} {b}").unwrap();
        return;
    }
    hanoi(a, 6 - a - b, n - 1, writer);
    writeln!(writer, "{a} {b}").unwrap();
    hanoi(6 - a - b, b, n - 1, writer);
}
