use proconio::input;
use std::cmp;
use std::collections::BTreeMap as map;
use std::collections::BTreeSet as set;
use std::collections::BinaryHeap as b_heap;
use std::collections::VecDeque as deque;

fn main() {
    input! {
        k: i32,
        x: i32,
    }

    let begin = cmp::max(-1000000, x - (k - 1));
    let end = cmp::min(1000000, x + (k - 1));

    print!("{}", begin);
    for i in (begin+1)..=end {
        print!(" {}", i);
    }

    println!("");
}
