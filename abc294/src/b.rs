use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }
    let alpha = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 0 {
                print!(".");
            } else {
                print!("{}", alpha.chars().nth(a[i][j] - 1).unwrap());
            }
        }
        println!("");
    }
}