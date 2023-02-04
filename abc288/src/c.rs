use proconio::input;

struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            rank: vec![0; n],
        }
    }
    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            let r = self.root(self.par[x]);
            self.par[x] = r;
            r
        }
    }
    fn unite(&mut self, x: usize, y: usize) {
        let x = self.root(x);
        let y = self.root(y);
        if x == y {
            return;
        }
        if self.rank[x] < self.rank[y] {
            self.par[x] = y;
        } else {
            self.par[y] = x;
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }
    fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut uf = UnionFind::new(n);
    let mut ans = 0;
    for (a, b) in ab {
        if !uf.same(a - 1, b - 1) {
            uf.unite(a - 1, b - 1);
            ans += 1;
        }
    }
    println!("{}", m - ans);
}