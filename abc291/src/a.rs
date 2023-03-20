// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        s: String,
    }
    let mut ans = 1;
    for c in s.chars().collect::<Vec<char>>() {
        if c.is_uppercase() {
            break;
        } else {
            ans += 1;
        }
    }
    println!("{}", ans);
}
