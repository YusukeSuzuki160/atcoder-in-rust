// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        n: usize,
        s: String,
    }
    let mut before = s.chars().nth(0).unwrap();
    for c in s.chars().skip(1) {
        if before == c {
            println!("No");
            return;
        }
        before = c;
    }
    println!("Yes");

}
