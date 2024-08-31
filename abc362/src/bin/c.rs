use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(i64, i64); n],
    }

    let mut ans = vec![0; n];
    let mut sum_l = 0;
    let mut sum_r = 0;

    for &(l, r) in &lr {
        sum_l += l;
        sum_r += r;
    }

    if sum_l > 0 || sum_r < 0 {
        println!("No");
        return;
    }

    for (i, &(l, _r)) in lr.iter().enumerate() {
        ans[i] = l;
    }

    println!("Yes");
    let mut sum = ans.iter().sum::<i64>();
    for i in 0..n {
        let d = (lr[i].1 - lr[i].0).min(-sum);
        ans[i] += d;
        sum += d;
        print!("{} ", ans[i]);
    }
    println!();
}

