use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut s: [String; n],
    }
    let mut anses = s[0..k].iter().collect::<Vec<&String>>();
    anses.sort_by(|a, b| a.cmp(b));
    for ans in anses {
        println!("{}", ans);
    }

}