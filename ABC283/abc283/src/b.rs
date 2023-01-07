use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
    };
    for i in 0..q {
        input! {
            t: usize,
        };
        if t == 1 {
            input! {
                x: usize,
                y: usize,
            }
            a[x - 1] = y;
        } else {
            input! {
                x: usize,
            }
            println!("{}", a[x - 1]);
        }
    }
}