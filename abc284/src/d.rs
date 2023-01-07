use proconio::input;

fn main() {
    input! {
        t: usize,
        tests: [u64; t],
    }

    for mut test in tests {
        for i in 1 as u64..test {
            if test % (i + 1) == 0 {
                test = test / (i + 1);
                if test % (i + 1) == 0 {
                    println!("{} {}", i + 1, test / (i + 1));
                    break;
                } else {
                    println!("{} {}", (test as f64).sqrt(), i + 1);
                    break;
                }
            }
        }
    }
}