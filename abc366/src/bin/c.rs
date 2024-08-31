use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        q: usize,
    }
    let mut h = HashMap::new();
    for _ in 0 .. q {
		input!{t: usize}
		if t == 1 {
			input!{x: usize}
			*h.entry(x).or_insert(0) += 1;
		} else if t == 2 {
			input!{x: usize}
			*h.get_mut(&x).unwrap() -= 1;
			if h[&x] == 0 {h.remove(&x);}
		} else {
			println!("{}", h.len())
		}
	}
}
