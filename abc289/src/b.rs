use proconio::input;

fn bfs(g: &Vec<Vec<usize>>, looker: &mut Vec<bool>, start: usize, ans: &mut Vec<usize>) {
    let mut que = std::collections::VecDeque::new();
    que.push_back(start);
    looker[start] = true;
    ans.push(start);
    while let Some(v) = que.pop_front() {
        for i in 0..g[v].len() {
            if looker[g[v][i]] {
                continue;
            }
            que.push_back(g[v][i]);
            ans.push(g[v][i]);
            looker[g[v][i]] = true;
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }
    if m == 0 {
        for i in 0..n {
            if i != 0 {
                print!(" ");
            }
            print!("{}", i + 1);
        }
        print!("\n");
        return;
    }
    let mut g = Vec::new();
    for _i in 0..n {
        g.push(Vec::new());
    }
    for i in 0..m {
        g[a[i] - 1].push(a[i]);
        g[a[i]].push(a[i] - 1);
    }
    /*for i in 0..n {
        println!("{:?}", g[i]);
    }*/
    let mut looker = Vec::new();
    for i in 0..n {
        looker.push(false);
    }
    let mut st = 0;
    loop {
        let mut ans = Vec::new();
        let mut end = 0;
        for i in 0..n {
            if looker[i] {
                if i == n - 1 {
                    end = 1;
                }
                continue;
            } else {
                bfs(&g, &mut looker, i, &mut ans);
                break;
            }
        }
        ans.sort_by(|a, b| b.cmp(a));
        for i in 0..ans.len() {
            if st != 0 {
                print!(" ");
            }
            print!("{}", ans[i] + 1);
            st = 1;
        }
        if end == 1 {
            print!("\n");
            break;
        }
    }
}