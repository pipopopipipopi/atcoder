use proconio::input;

fn main() {
    input! {
        r: usize,
        g: usize,
        b: usize,
        c: String,
    }
    if c == "Red" {
        println!("{}", g.min(b));
    } else if c == "Blue" {
        println!("{}", r.min(g));
    } else {
        println!("{}", r.min(b));
    }
}
