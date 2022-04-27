use proconio::input;
use std::cmp;
use std::collections::BTreeMap as map;
use std::collections::BTreeSet as set;
use std::collections::BinaryHeap as b_heap;
use std::collections::VecDeque as deque;

fn main() {
    input! {
        x: i64,
    }

    let res = if x < 0 && (x % 10) != 0 {
        (x / 10) - 1
    } else {
        x / 10
    };
    println!("{}", res)
}
