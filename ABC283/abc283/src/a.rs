// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        a: usize,
        b: isize,
    }
    println!("{}", a.pow(b as u32));
}
