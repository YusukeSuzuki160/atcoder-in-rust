use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut x = Vec::new();
    for _ in 0.. 5 * n {
        input! {
            a: i32,
        }
        x.push(a);
    }
    x.sort();
    let mut ans = 0;
    for i in n..4 * n {
        ans += x[i];
    }
    println!("{}", ans as f64 / (3 * n) as f64);
}