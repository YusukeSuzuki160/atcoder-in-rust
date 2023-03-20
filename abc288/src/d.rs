use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
        q: usize,
        lr: [(usize, usize); q],
    }
    let mut ds = Vec::new();
    for i in 0..n + 1 {
        if i == 0 {
            ds.push(a[i]);
        } else if i == n {
            ds.push(-a[i - 1]);
        } else {
            ds.push(a[i] - a[i - 1]);
        }
    }
    let mut sums = Vec::new();
    for i in 0..k {
        let mut sum = Vec::new();
        let mut s = 0;
        for j in 0..n + 1 {
            if j % k == i {
                s += ds[j];
            }
            sum.push(s);
        }
        sums.push(sum);
    }
    for (l, r) in lr {
        let mut ans = "Yes";
        let start = (l - 1) % k;
        let end = r % k;
        for i in 0..k {
            let mut s = 0;
            if l > 1 {
                s += sums[i][r] - sums[i][l - 2];
            } else {
                s += sums[i][r];
            }
            if i == start && l > 1 {
                s += a[l - 2];
            }
            if i == end && r < n {
                s -= a[r];
            }
            if s != 0 {
                ans = "No";
                break;
            }
        }
        println!("{}", ans);
    }
}