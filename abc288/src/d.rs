use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
        q: usize,
        lr: [(usize, usize); q],
    }
    for (l, r) in lr {
        let mut ans = Vec::new();
        for i in l - 1..r - 1 {
            ans.push(a[i]);
        }

        if ans[l - r] == 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}