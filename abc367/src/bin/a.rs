use proconio::input;

fn main() {
    input! {
        abc: [usize; 3]
    }
    let mut h = vec![0; 24];
    let mut cnt = 1;
    for i in &abc {
        h[*i] = cnt;
        cnt += 1;
    }
    let mut flag = false;
    let mut index = abc[2];
    loop {
        if h[index % 24] == 2 {
            break;
        } else if h[index % 24] == 1 {
            flag = true;
        }
        index += 1;
    }
    if flag {
        println!("Yes");
    } else {
        println!("No");
    }
}
