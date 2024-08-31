use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        w: [usize; n]
    }
    let mut hako = vec![vec![]; n];
    let mut ans = 0;
    for i in 0..n {
        hako[a[i] - 1].push(w[i]);
    }
    for i in 0..n {
        if !hako[i].is_empty() {
            let max_value = *hako[i].iter().max().unwrap();
            let max_pos = hako[i].iter().position(|&x| x == max_value).unwrap();
            hako[i].remove(max_pos);
            ans += hako[i].iter().sum::<usize>();
        }
    }
    println!("{}", ans);
}
