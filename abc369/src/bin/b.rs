use proconio::input;

fn main() {
    input! {
        n: usize,
        a_s: [(isize, char); n]
    }
    let mut lr = (0, 0);
    let mut ans = 0;
    for i in 0..n {
        if a_s[i].1 == 'L' {
            lr.0 = a_s[i].0;
            break;
        }
    }
    for i in 0..n {
        if a_s[i].1 == 'R' {
            lr.1 = a_s[i].0;
            break;
        }
    }
    for i in 0..n {
        if a_s[i].1 == 'L' {
            ans += (lr.0 - a_s[i].0).abs();
            lr.0 = a_s[i].0;
        } else if a_s[i].1 == 'R' {
            ans += (lr.1 - a_s[i].0).abs();
            lr.1 = a_s[i].0;
        }
    }
    println!("{}", ans);
}
