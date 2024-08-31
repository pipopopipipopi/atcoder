use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [i64; n]
    }
    let mut ans = 0;
    for i in h {
        let mut tmp = i;
        ans += (tmp / 5) * 3;
        tmp -= (tmp / 5) * 5;
        while tmp > 0 {
            ans += 1;
            if ans % 3 == 0 {
                tmp -= 3;
            } else {
                tmp -= 1;
            }
        }
    }
    println!("{}", ans);
}
