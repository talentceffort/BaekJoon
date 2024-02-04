use std::io::Write;
use std::io::{self, BufWriter, Read};
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_ascii_whitespace().flat_map(str::parse::<usize>);

    let n: usize = input.next().unwrap();
    let r: usize = input.next().unwrap();
    let c: usize = input.next().unwrap();

    let result = func(n, r, c);

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    writeln!(writer, "{result}").unwrap();
}

fn func(n: usize, r: usize, c: usize) -> usize {
    if n == 0 {
        return 0;
    }

    let half = 1 << (n - 1);

    // (0, 0)
    if r < half && c < half {
        return func(n - 1, r, c);
    }
    // (0, 1)
    if r < half && c >= half {
        return half * half + func(n - 1, r, c - half);
    }
    // (1, 0)
    if r >= half && c < half {
        return 2 * half * half + func(n - 1, r - half, c);
    }
    // (1, 1)
    return 3 * half * half + func(n - 1, r - half, c - half);
}
