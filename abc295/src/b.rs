use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
        mut bs: [String; r],
    }
    let mut ans = Vec::new();
    for b in bs.clone() {
        ans.push(b.chars().collect::<Vec<char>>());
    }
    for i in 0..r {
        for j in 0..c {
            if let Ok(n) = bs[i].chars().nth(j).unwrap().to_string().parse::<usize>() {
                for k in 0..r {
                    for l in 0..c {
                        if bs[k].chars().nth(l).unwrap() == '#' {
                            if calc_dist((i, j), (k, l)) <= n {
                                ans[k][l] = '.';
                            } else {
                            }
                        } else {
                        }
                    }
                }
                ans[i][j] = '.';
            }
        }
    }
    for i in 0..r {
        for j in 0..c {
            print!("{}", ans[i][j]);
        }
        println!();
    }
}

fn calc_dist(z1: (usize, usize), z2: (usize, usize)) -> usize {
    let (x1, y1) = z1;
    let (x2, y2) = z2;
    ((x1 as isize - x2 as isize).abs() + (y1 as isize - y2 as isize).abs()) as usize
}