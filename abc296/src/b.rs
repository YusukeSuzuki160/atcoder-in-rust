use proconio::input;

fn main() {
    input! {
        s: [String; 8]
    }
    let row = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    for i in 0..8 {
        for j in 0..8 {
            if s[i].chars().nth(j).unwrap() == '*' {
                println!("{}{}", row[j], 8 - i)
            }
        }
    }
}