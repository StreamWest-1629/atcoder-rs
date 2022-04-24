use proconio::input;
use std::cmp;
use std::collections::BTreeMap as map;
use std::collections::BTreeSet as set;
use std::collections::BinaryHeap as b_heap;
use std::collections::VecDeque as deque;

fn main() {
    let comp: i32 = 753;
    let mut nearest: i32 = 0;
    let mut nearest_diff: i32 = comp;

    input! {
        st: String,
    }

    let s: &str = &st;

    let v = s.chars();
    for i in 0..(s.len() - 2) {
        let check: i32 = s[i..i + 3].parse().unwrap();
        let diff = (check - comp).abs();
        if nearest_diff > diff {
            nearest = check;
            nearest_diff = diff;
        }
    }

    println!("{}", nearest_diff)
}
