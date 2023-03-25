use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
    }
    a.sort();
    let mut ans = 0;
    let mut num = 1;
    let mut last = a[0];
    for i in 1..n {
        if last == a[i] {
            num += 1;
        } else {
            ans += num / 2;
            num = 1;
            last = a[i];
        }
        //println!("{} {} {}", ans, num, last);
    }
    ans += num / 2;
    println!("{}", ans);
}