use proconio::input;
use std::cmp;
use std::collections::BTreeMap as map;
use std::collections::BTreeSet as set;
use std::collections::BinaryHeap as b_heap;
use std::collections::VecDeque as deque;

fn main() {
    input! {
        h: f64
    }

    println!("{}", (h * (12800000.0 + h)).sqrt())
}
