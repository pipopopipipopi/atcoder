use proconio::input;
use proconio::marker::Chars;

fn main() {
	input!{s: Chars, t: Chars}
	for i in 1 .. s.len() {
		let mut v = vec![vec![]; i];
		for j in 0 .. s.len() {
			v[j % i].push(s[j]);
		}
		if v.iter().any(|w| w == &t) {
			println!("Yes");
			return;
		}
	}
	println!("No");
}

