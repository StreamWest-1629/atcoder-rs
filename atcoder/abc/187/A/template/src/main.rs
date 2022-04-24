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

    let a_res = (a / 100) + ((a % 100) / 10) + (a % 10);
    let b_res = (b / 100) + ((b % 100) / 10) + (b % 10);
    println!("{}", cmp::max(a_res, b_res));
}
