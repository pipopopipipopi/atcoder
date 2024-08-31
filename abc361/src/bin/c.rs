use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n]
    }
    a.sort_by(|a, b| b.cmp(a));
    let mut ans = usize::MAX;
    for i in 0..=k {
        let sub = a[i] - a[i + (n - k) - 1];
        ans = ans.min(sub);
    }
    println!("{}", ans);
}
