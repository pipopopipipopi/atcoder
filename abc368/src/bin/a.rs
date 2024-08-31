use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    for i in &a[n - k..n] {
        print!("{} ", i);
    }
    for i in &a[0..n - k] {
        print!("{} ", i);
    }

    println!();
}
