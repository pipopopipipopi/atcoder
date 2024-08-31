use proconio::input;
use proconio::marker::Chars;
use permutohedron::LexicalPermutation;

fn main() {
    input! {
        _n: usize,
        k: usize,
        mut s: Chars
    }
    let mut cnt = 0;
    s.sort();
    loop {
        if !s.windows(k).any(|w| (0..k / 2).all(|i| w[i] == w[k - i -1])) {
            cnt += 1;
        }

        if !s.next_permutation() {
            break;
        }
    }
    println!("{cnt}");
}
