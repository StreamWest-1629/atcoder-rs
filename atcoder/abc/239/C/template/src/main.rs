use proconio::input;
use std::cmp;
use std::collections::BTreeMap as map;
use std::collections::BTreeSet as set;
use std::collections::BinaryHeap as b_heap;
use std::collections::VecDeque as deque;

fn main() {
    input! {
        pairs: [(i64, i64); 2],
    }

    let checks: Vec<(i64, i64)> = vec![
        (1, 2),
        (2, 1),
        (-1, 2),
        (2, -1),
        (1, -2),
        (-2, 1),
        (-1, -2),
        (-2, -1),
    ];

    for (x, y) in checks.iter() {
        if (pairs[1].0 - pairs[0].0 + x).pow(2) + (pairs[1].1 - pairs[0].1 + y).pow(2) == 5 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
