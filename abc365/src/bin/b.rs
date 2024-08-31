use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut tmp = a.to_vec();
    tmp.sort_by(|a, b| b.cmp(a));

    let mut ans = 1;
    for i in a {
        if tmp[1] == i {
            break;
        }
        ans += 1;
    }
    println!("{}", ans);
}
