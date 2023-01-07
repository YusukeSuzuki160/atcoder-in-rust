use proconio::input;

const MAX: usize = 1000000;

fn dfs(graph: &Vec<Vec<usize>>, v: usize, mut visited: &mut Vec<bool>) -> usize {
    visited[v] = true;
    let mut ans = 1;
    for &next in graph[v].iter() {
        if visited[next] {
            continue;
        }
        ans += dfs(graph, next, &mut visited);
        if ans >= MAX {
            println!("{}", MAX);
            std::process::exit(0);
        }
    }
    visited[v] = false;
    //println!("{} {}", v, ans);

    ans
}

fn main() {
    input! {
        n: usize,
        m: usize,
        edge: [(usize, usize); m],
    }
    let mut g = vec![vec![]; n];
    for (a, b) in edge {
        g[a - 1].push(b - 1);
        g[b - 1].push(a - 1);
    }
    let mut visited = vec![false; n];
    let mut ans;
    ans = dfs(&g, 0, &mut visited);
    if ans > MAX {
        ans = MAX;
    }
    println!("{}", ans);

}