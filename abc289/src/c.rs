use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut a = Vec::new();
    let mut c = Vec::new();
    for i in 0..m {
        input! {
            ci: usize,
            ai: [usize; ci],
        }
        c.push(ci);
        a.push(ai);
    }
    let mut ans = 0;
    let mut exist = Vec::new();
    for i in 0..m {
        exist.push(Vec::new());
        for j in 0..n {
            exist[i].push(false);
        }
    }
    for i in 0..m {
        for j in 0..c[i] {
            exist[i][a[i][j] - 1] = true;
        }
    }
    let MAX_COUNT = 2usize.pow(m as u32);
    for bit in 0..MAX_COUNT {
        let sub_list = (0..m).filter(|x| (bit & (1 << x) as usize) != 0).collect::<Vec<usize>>();
        let mut current_exist = Vec::new();
        for i in 0..n {
            current_exist.push(false);
        }
        if sub_list.len() == 0 {
            continue;
        }
        for i in 0..n {
            for j in 0..sub_list.len() {
                if exist[sub_list[j]][i] {
                    current_exist[i] = true;
                }
            }
        }
        for i in 0..n {
            if !current_exist[i] {
                break;
            }
            if i == n - 1 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);

}