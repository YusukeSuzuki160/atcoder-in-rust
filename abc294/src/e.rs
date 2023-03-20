use proconio::input;

fn main() {
    input! {
        l: u64,
        n1: usize,
        n2: usize,
        vl1: [(u64, u64); n1],
        vl2: [(u64, u64); n2],
    }
    let mut ans = 0;
    let mut range1 = Vec::new();
    let mut range2 = Vec::new();
    range1.push((0, 0));
    range2.push((0, 0));
    for i in 0..n1 {
        range1.push((vl1[i].0, vl1[i].1 + range1[i].1));
    }
    for i in 0..n2 {
        range2.push((vl2[i].0, vl2[i].1 + range2[i].1));
    }
    let mut current = 1;
    for i in 1..(n1 + 1) {
        loop {
            let lefta = range1[i - 1].1;
            let leftb = range2[current - 1].1;
            let righta = range1[i].1;
            let rightb = range2[current].1;
            let valuea = range1[i].0;
            let valueb = range2[current].0;
            if valuea == valueb {
                let right = std::cmp::min(righta, rightb);
                let left = std::cmp::max(lefta, leftb);
                ans += right - left;
            }
            if righta > rightb {
                current += 1;
            } else if righta == rightb {
                current += 1;
                break;
            } else {
                break;
            }
        }
    }
    println!("{}", ans);
}
