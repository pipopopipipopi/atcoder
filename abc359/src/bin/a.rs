use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }
    let mut ans = 0;
    for i in s {
        if i == "Takahashi" {
            ans += 1;
        }
    }
    println!("{}", ans);
}
