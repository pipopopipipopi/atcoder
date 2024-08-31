use proconio::input;

fn main() {
    input! {
        a :i64,
        b :i64,
        c :i64,
        d :i64,
        e :i64,
        f :i64,
        g :i64,
        h :i64,
        i :i64,
        j :i64,
        k :i64,
        l :i64,
    }
    let x = (a.max(g), b.max(h), c.max(i));
    let y = (d.min(j), e.min(k), f.min(l));
    if (y.0 - x.0) * (y.1 - x.1) * (y.2 - x.2) > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
