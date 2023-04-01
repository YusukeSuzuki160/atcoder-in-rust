use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        n: usize,
        x: i64,
        mut a: [i64; n],
    }
    a.sort();
    for i in 0..n {
        let x2 = a.lower_bound(&(a[i] + x));
        if x2 < n && a[x2] == a[i] + x {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();
        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                },
                Ordering::Greater | Ordering::Equal => {
                    high = mid;
                },
            }
        }
        low
    }
    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low != high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less | Ordering::Equal => {
                    low = mid + 1;
                },
                Ordering::Greater => {
                    high = mid;
                },
            }
        }
        low
    }
}