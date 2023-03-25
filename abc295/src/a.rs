// -*- coding:utf-8-unix -*-

use std::string::ToString;
use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        n: usize,
        ws: [String; n]
    }
    let target = vec!["and".to_string(), "not".to_string(), "that".to_string(), "the".to_string(), "you".to_string()];
    for w in ws {
        if target.contains(&w) {
            println!("Yes");
            return;
        }
    }
    println!("No");

}
