use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: (Usize1, Usize1),
        c: [Chars; h],
        x: Chars
    }
    let mut xy = (s.1, s.0);
    for i in x {
        if i == 'U' && xy.1 != 0 && c[xy.1 - 1][xy.0] != '#' {
            xy.1 -= 1;
        } else if i == 'D' && xy.1 != (h - 1) && c[xy.1 + 1][xy.0] != '#' {
            xy.1 += 1;
        } else if i == 'L' && xy.0 != 0 && c[xy.1][xy.0 - 1] != '#' {
            xy.0 -= 1;
        } else if i == 'R' && xy.0 != (w - 1) && c[xy.1][xy.0 + 1] != '#' {
            xy.0 += 1;
        }
        eprintln!("{:?}", s);
    }
    println!("{} {}", xy.1 + 1, xy.0 + 1);
}
