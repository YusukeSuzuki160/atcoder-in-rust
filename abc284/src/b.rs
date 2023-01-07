use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _i in 0..t {
        input! {
            n: usize,
            a: [u64; n],
        }
        let mut ans = 0;
        for _j in 0..n {
            if a[_j] % 2 == 1 {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}