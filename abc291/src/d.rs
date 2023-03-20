use proconio::input;

const MOD: u64 = 998244353;

fn main() {
    input! {
        n: usize,
        ab: [(u64, u64); n],
    }
    let mut ans = Vec::new();
    for _ in 0..n {
        ans.push(0);
    }
    let mut current_num = None;
    let mut current_num0 = None;
    let mut current_num1 = None;
    ans[0] = 2;
    for i in 0..n - 1 {
        let (a0, b0) = ab[i];
        let (a1, b1) = ab[i + 1];
        let mut same_num0 = None;
        let mut same_num1 = None;
        let mut same_num = None;
        if a0 == a1 {
            same_num0 = Some(a0);
        } if a0 == b1 {
            same_num0 = Some(a0);
        } if b0 == a1 {
            same_num1 = Some(b0);
        } if b0 == b1 {
            same_num1 = Some(b0);
        }
        if same_num0 != None && same_num1 != None {
            ans[i + 1] = ans[i];
            current_num0 = same_num0;
            current_num1 = same_num1;
            current_num = same_num0;
            continue;
        }
        if same_num0 != None {
            same_num = same_num0;
        } else if same_num1 != None {
            same_num = same_num1;
        }
        if same_num0 == current_num0 || same_num0 == current_num1 {
            current_num = same_num0;
        } else if same_num1 == current_num1 || same_num1 == current_num0 {
            current_num = same_num1;
        }
        if same_num == None {
            ans[i + 1] = ans[i] * 2 % MOD;
            current_num = None;
        } else {
            if current_num == None {
                if i == 0 {
                    ans[i + 1] = 3;
                } else {
                    ans[i + 1] = ans[i] * 3 / 2 % MOD;
                }
                current_num = same_num;
            } else {
                if current_num == same_num {
                    ans[i + 1] = (ans[i] % MOD + ans[i - 1] % MOD) % MOD;
                } else {
                    if i >= 2 {
                        ans[i + 1] = (ans[i] % MOD + ans[i - 2] % MOD) % MOD;
                    } else {
                        ans[i + 1] = (ans[i] + 1) % MOD;
                    }
                        current_num = None;
                    }
                }
            }
        }
        /*for a in ans.iter() {
            println!("{}", a);
        }*/
        println!("{}", ans[n - 1]);
    }