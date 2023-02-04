// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n],
    }
    for i in 0..n {
        let (a, b) = ab[i];
        println!("{}", a + b)
    }

}
