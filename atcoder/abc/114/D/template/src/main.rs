use proconio::input;
use std::cmp;
use std::collections::BTreeMap as map;
use std::collections::BTreeSet as set;
use std::collections::BinaryHeap as b_heap;
use std::collections::VecDeque as deque;

fn main() {
    input! {
        n: i32,
    }

    let mut n_divs = 0;

    for val in 2..(n + 1) {
        let mut buf = val;
        let mut div = 2;
        while buf > 1 {
            while buf % div == 0 {
                buf /= div;
                n_divs += 1;
            }
            div += 1;
        }
    }

    if n_divs < 75 {
        println!("0");
        return;
    } else {
    }
}
