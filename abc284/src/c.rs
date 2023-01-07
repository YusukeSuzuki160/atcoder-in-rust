use proconio::input;

fn dfs(graph: &Vec<Vec<usize>>, v: usize, visited: &mut Vec<bool>) {
    visited[v] = true;
    for &next in graph[v].iter() {
        if visited[next] {
            continue;
        }
        dfs(graph, next, visited);
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        e: [(usize, usize); m],
    }

    let mut graph = vec![vec![]; n];
    for (a, b) in e {
        graph[a - 1].push(b - 1);
        graph[b - 1].push(a - 1);
    }

    let mut visited = vec![false; n];
    let mut ans = 0;
    for i in 0..n {
        if visited[i] {
            continue;
        }
        dfs(&graph, i, &mut visited);
        ans += 1;
    }
    println!("{}", ans);
}