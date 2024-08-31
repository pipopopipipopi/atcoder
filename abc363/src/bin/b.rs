use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        p: usize,
        mut l: [usize; n]
    }
    let mut cnt = 0;
    
    while l.iter().filter(|&&x| x >= t).count() < p {
        for i in l.iter_mut() {
            if *i < t {
                *i += 1;
            }
        }
        cnt += 1;
    }

    println!("{}", cnt);
}

