use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u128; n],
        c: [u128; n],
        x: [u128; m],
    }
    if n == 1 {
        println!("{}", c[0] + a[0]);
        return;
    }
    let mut cost: Vec<Vec<u128>> = Vec::new();
    for _i in 0..n + 1 {
        cost.push(Vec::new());
    }
    for i in 1..n + 1 {
        for j in 0..i {
            if j == 0 {
                cost[i].push(c[i - 1]);
            } else  {
                if c[(i as isize - j as isize - 1) as usize] < cost[i][j - 1] {
                    cost[i].push(c[(i as isize - j as isize - 1) as usize]);
                } else {
                    let tmp = cost[i][j - 1].clone();
                    cost[i].push(tmp);
                }
            }
        }
    }
    /*for i in 0..n + 1 {
        println!("{:?}", cost[i]);
    }*/
    let mut dp: Vec<Vec<u128>> = Vec::new();
    for _i in 0..n + 1 {
        dp.push(Vec::new());
    }
    for i in 0..n + 1 {
        for _j in 0..n + 1 {
            dp[i].push(18446744073709551615);
        }
    }
    dp[0][0] = 0;
    for i in 0..n {
        if i == 0 && x.iter().any(|&x| x == 1) == false {
            dp[1][i] = 0;
        } else if i == 1 {
            dp[1][i] = a[0] + cost[1][i - 1];
        } else {
            dp[1][i] = 18446744073709551615;
        }
    }
    for i in 1..n {
        let flag = x.iter().any(|&x| x == i as u128 + 1);
        if flag == false {
            dp[i + 1][0] = dp[i][0];
        } else {
            dp[i + 1][0] = 18446744073709551615;
        }
        for j in 0..i + 1 {
            if dp[i + 1][j + 1] > dp[i][j] + a[i] + cost[i + 1][j] {
                dp[i + 1][j + 1] = dp[i][j] + a[i] + cost[i + 1][j];
            } else {
                dp[i + 1][j + 1] = dp[i + 1][j + 1];
            }
            if dp[i + 1][j] > dp[i][j] && flag == false {
                dp[i + 1][j] = dp[i][j];
            } else {
                dp[i + 1][j] = dp[i + 1][j];
            }
        }
    }

    let mut ans: u128 = 18446744073709551615;
    for i in m..n + 1 {
        if ans > dp[n][i] {
            ans = dp[n][i];
        }
    }
    /*for i in 1..n + 1 {
        println!("{:?}", dp[i]);
    }*/
    println!("{}", ans);
}