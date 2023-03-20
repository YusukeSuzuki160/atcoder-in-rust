// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        s: String,
    }
    let mut ans: Vec<char> = s.chars().collect();
    for c in ans.iter_mut() {
        if *c == '0' {
            print!("1")
        } else {
            print!("0")
        }
    }
    println!("");

}
