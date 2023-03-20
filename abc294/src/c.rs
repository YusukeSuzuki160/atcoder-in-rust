use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u64; n],
        b: [u64; m],
    }
    let mut a2 = Vec::new();
    let mut b2 = Vec::new();
    for i in 0..n {
        a2.push((a[i], i + 1));
    }
    for i in 0..m {
        b2.push((b[i], i + 1 + n));
    }
    let mut c = Vec::new();
    c.append(&mut a2);
    c.append(&mut b2);
    c.sort_by(|a, b| a.0.cmp(&b.0));
    let mut ans = Vec::new();
    for i in 0..n + m {
        ans.push(0);
    }
    for i in 0..n + m {
        ans[c[i].1 - 1] = i + 1;

    }
    for i in 0..n {
        if i != 0 {
            print!(" ");
        }
        print!("{}", ans[i]);
    }
    println!("");
    for i in 0..m {
        if i != 0 {
            print!(" ");
        }
        print!("{}", ans[i + n]);
    }
    println!("");
}