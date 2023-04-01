use proconio::input;

fn main() {
    input! {
        n: u64,
        mut m: u64,
    }
    if m > n.pow(2) {
        println!("-1");
    } else {
        loop {
            let div = divisor(m);
            if m / div.last().unwrap() <= n {
                println!("{}", m);
                return;
            }
            m += 1;
        }
    }
}

fn divisor(n: u64) -> Vec<u64> {
    let mut res = vec![];
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            res.push(i);
        }
        i += 1;
    }
    res
}