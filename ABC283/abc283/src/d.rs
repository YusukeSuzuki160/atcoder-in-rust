use proconio::input;
use std::collections::{HashSet, HashMap};

fn main() {
    input! {
        s: String,
    }
    let cs = s.chars().collect::<Vec<char>>();
    let mut x : Vec<HashSet<char>> = Vec::new();
    let mut c = 0;
    let mut used = HashMap::new();
    let alphalist = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>();
    for _i in 0..cs.len() {
        x.push(HashSet::new());
    }
    for alphabet in alphalist {
        used.insert(alphabet, false);
    }
    for ch in cs {
        if ch == '(' {
            c += 1;
        } else if ch == ')' {
            for cl in &x[c] {
                used.insert(*cl, false);
            }
            x[c].clear();
            c -= 1;
        } else {
            if used[&ch] {
                println!("No");
                return;
            }
            x[c].insert(ch);
            used.insert(ch, true);
        }
    }
    println!("Yes");
}