use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    eprintln!("{:?}", n);
    eprintln!("{:?}", s);
    let mut flag = true;
    let mut cnt = 0;
    for i in s {
        if cnt == 2 {
            flag = false;
        }
        if i == "sweet" {
            cnt += 1;
        } else {
            cnt = 0;
        }
    }
    
    if flag {
        println!("Yes");
    } else {
        println!("No");
    }
}
