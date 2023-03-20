use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }
    let mut left = 0;
    let mut right = 0;
    let mut up = 0;
    let mut down = 0;
    for c in s.chars() {
        match c {
            'L' => left += 1,
            'R' => right += 1,
            'U' => up += 1,
            'D' => down += 1,
            _ => (),
        }
        //println!("{} {} {} {}", left, right, up, down);
        if left == right && up == down {
            println!("Yes");
            return;
        }
    }
    println!("No");
}