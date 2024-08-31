use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }
    let mut ans = 0;
    while a.len() > 1 {
        a.sort_by(|a, b| b.cmp(a));
        a[0] -= 1;
        a[1] -= 1;
        a.retain(|&x| x > 0);
        ans += 1;
    }
    println!("{}", ans);
}
