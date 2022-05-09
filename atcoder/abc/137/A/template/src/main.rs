use proconio::input;
use std::cmp;
use std::collections::BTreeMap as map;
use std::collections::BTreeSet as set;
use std::collections::BinaryHeap as b_heap;
use std::collections::VecDeque as deque;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    println!("{}", cmp::max(a + b, cmp::max(a - b, a * b)));
}
