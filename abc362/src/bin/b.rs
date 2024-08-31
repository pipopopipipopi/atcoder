use proconio::input;

fn main() {
    input! {
        a: (f32, f32),
        b: (f32, f32),
        c: (f32, f32)
    }
    if (a.0 - b.0).abs() / (a.1 - b.1).abs() == (b.1 - c.1).abs() / (b.0 - c.0).abs()
        || (b.0 - c.0).abs() / (b.1 - c.1).abs() == (c.1 - a.1).abs() / (c.0 - a.0).abs()
        || (c.0 - a.0).abs() / (c.1 - a.1).abs() == (a.1 - b.1).abs() / (a.0 - b.0).abs()
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
