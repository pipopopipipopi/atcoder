use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    }
    let mut flag = true;
    for i in s {
        if i == 'M' {
            flag = false;
        } else if flag && i == 'R' {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
