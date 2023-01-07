// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    for i in (0..n).rev() {
        println!("{}", s[i]);
    }
}
