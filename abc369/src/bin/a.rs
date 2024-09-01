use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }
    let mut ans = 0;
    if (a - b).abs() > 0 {
        ans += 2;
    }
    for i in a.min(b)..=a.max(b) {
        if a - i == i - b {
            ans += 1;
        }
    }
    println!("{}", ans);
}
