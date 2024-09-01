use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n]
    }
    let mut ans = n;
    let mut sub = vec![0; n - 1];
    for i in 0..n - 1 {
        sub[i] = a[i + 1] - a[i];
    }
    let mut rlc = vec![];
    for i in &sub {
        match rlc.last_mut() {
            Some((last_i, count)) if i == *last_i => *count += 1,
            _ => rlc.push((i, 1)),
        }
    }
    for i in rlc {
        ans += (i.1 * (i.1 + 1)) / 2;
    }
    println!("{}", ans);
}
