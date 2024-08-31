use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        k: usize,
        r: [usize; n]
    }
    let mut tmp = vec![1; n];
    let mut ans = vec![vec![]];
    let max_value = *r.iter().max().unwrap_or(&1);
    loop {
        let sum: usize = tmp.iter().sum();
        if sum % k == 0 {
            ans.push(tmp.clone());
        }

        let mut i = n;
        while i > 0 {
            i -= 1;
            if tmp[i] < max_value {
                tmp[i] += 1;
                break;
            }
            tmp[i] = 1;
        }

        if i == 0 && tmp[0] == 1 {
            break;
        }
    }

    let mut flag = true;
    for i in ans {
        flag = true;
        if i.len() > 0 {
            for j in 0..n {
                if i[j] > r[j] {
                    flag = false;
                    break;
                }
            }
            if flag {
                for k in 0..n {
                    print!("{} ", i[k]);
                }
                println!();
            }
        }
    }

}
