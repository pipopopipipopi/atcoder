use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n]
    }
    a.sort();
    if m >= a.iter().sum() {
        println!("infinite");
    } else {
        println!("{}", b_search(&a, m));
    }
}

fn question(a: &[usize], mid: usize, k: usize) -> bool {
    let mut ans = 0;
    for &i in a {
        ans += i.min(mid);
    }
    eprintln!("{}", ans);
    
    ans > k
}

fn b_search(a: &[usize], k: usize) -> usize {
    let mut ng = 0;
    let mut ok = *a.iter().max().unwrap();
    while ok - ng > 1 {
        let mid = (ng + ok) / 2;
        if question(a, mid, k) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ng
}

