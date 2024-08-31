use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [[usize; 2]; n],
    };
    let mut dp: Vec<Vec<i32>> = vec![vec![-1; n + 1]; w + 1];
    dp[0][0] = 0;
    for i in 1..=n {
    }
}
