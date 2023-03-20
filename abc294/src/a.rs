// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/tasks/arc089_a

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut start = 0;
    for b in a {
        if b % 2 == 0 {
            if start == 1 {
                print!(" ");
            }
            print!("{}", b);
            start = 1;
        }
    }
    println!("");

}
