use std::clone::Clone;
use proconio::input;
use std::collections::HashSet;
use std::string::ToString;

fn main() {
    input! {
        s: String,
    }
    let mut not_pair = HashSet::new();
    for c in s.clone().chars() {
        if not_pair.contains(&c) {
            not_pair.remove(&c);
        } else {
            not_pair.insert(c);
        }
    }
    let mut not_pair_list = Vec::new();
    not_pair_list.push(to_list(&not_pair));
    for c in s.chars() {
        if not_pair.contains(&c) {
            not_pair.remove(&c);
        } else {
            not_pair.insert(c);
        }
        not_pair_list.push(to_list(&not_pair));
    }
    //not_pair_list.push(Vec::new());
    /*for l in not_pair_list.clone() {
        println!("{:?}", l);
    }*/
    let mut ans: u128 = 0;
    not_pair_list.sort();
    let mut count = 0;
    let mut prev = not_pair_list[0].clone();
    for not_pair in not_pair_list {
        if prev == not_pair {
            count += 1;
        } else {
            if count > 0 {
                ans += count * (count - 1) / 2;
            }
            count = 1;
            prev = not_pair;
        }
    }
    if count > 0 {
        ans += count * (count - 1) / 2;
    }
    println!("{}", ans);
}

fn to_list(a: &HashSet<char>) -> Vec<usize> {
    let mut list = Vec::new();
    for c in a {
        list.push(c.to_string().parse::<usize>().unwrap());
    }
    list.sort();
    list
}