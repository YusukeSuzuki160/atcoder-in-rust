use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut ans = Vec::new();
    let mut non_called = BinaryHeap::new();
    let mut called = BinaryHeap::new();
    let mut is_go = vec![false; n];
    for i in 0..n {
        non_called.push(-(i as i64));
    }
    for _ in 0..q {
        input! {
            n: usize,
        }
        if n == 1 {
            let call = non_called.pop().unwrap();
            called.push(call);
        } else if n == 2 {
            input! {
                x: usize,
            }
            is_go[x - 1] = true;
        } else {
            let mut call = called.pop().unwrap();
            while is_go[-call as usize] {
                call = called.pop().unwrap();
            }
            ans.push(-call + 1);
        }
    }
    for a in ans {
        println!("{}", a);
    }
}