use proconio::input;

fn main() {
    input! {
        mut n: usize,
        t: usize,
        a: usize
    }
    n = n - (t + a);
    eprintln!("{n}");
    if (n + t > a && n + a < t) || (n + t < a && n + a > t) {
        println!("Yes");
    } else {
        println!("No");
    }
}
