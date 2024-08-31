use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        mut a: [usize; n],
        mut b: [usize; n]
    }
    a.sort_by(|a, b| b.cmp(a));
    b.sort_by(|a, b| b.cmp(a));
    let mut a_sum = 0;
    let mut b_sum = 0;
    let mut cnt = 0;
    for (i, j) in a.iter().zip(b.iter()) {
        a_sum += i;
        b_sum += j;
        eprintln!("{} {}", a_sum, b_sum);
        cnt += 1;
        if x < a_sum || y < b_sum {
            break;
        }
    }
    println!("{}", cnt);

}
