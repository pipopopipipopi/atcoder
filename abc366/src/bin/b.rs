use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    eprintln!("{:?}", s);
    eprintln!("{:?}", s.iter().max_by_key(|v| v.len()).map_or(0, |v| v.len()));
    let max_len = s.iter().max_by_key(|v| v.len()).map_or(0, |v| v.len());
    let mut tmp = vec![vec!['*'; n]; max_len];
    eprintln!("{:?}", tmp);
    for i in 0..max_len {
        for j in 0..n {
            if i < s[n - j - 1].len() {
                tmp[i][j] = s[n - j - 1][i];
            }
        }
    }
    for row in tmp {
        let mut row_string = row.iter().collect::<String>();
        
        while row_string.ends_with('*') {
            row_string.pop();
        }
        
        println!("{}", row_string);
    }
}
