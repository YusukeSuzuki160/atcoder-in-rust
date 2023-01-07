use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let mut ans = 0;
    let a = s.chars().collect::<Vec<char>>();
    let mut i = a.len() as isize - 1 as isize;
    while i >= 0 {
        if i > 0 && a[i as usize] == '0' && a[i as usize - 1] == '0' {
            ans += 1;
            i -= 2;
        } else {
            ans += 1;
            i -= 1;
        }
    }
    println!("{}", ans);
}