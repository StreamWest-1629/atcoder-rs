use proconio::input;
use std::cmp;
use std::collections::BTreeMap as map;
use std::collections::BTreeSet as set;
use std::collections::BinaryHeap as b_heap;
use std::collections::VecDeque as deque;

fn main() {
    input! {
        n: i32,
        pos: [(i32, i32); n],
    };

    let mut ans = 0;

    for i in 0..pos.len() {
        for j in (i + 1)..pos.len() {
            let (i_x, i_y) = pos[i];
            let (j_x, j_y) = pos[j];
            if (i_x - j_x).abs() >= (i_y - j_y).abs() {
                ans += 1;
            }
        }
    }

    println!("{}", ans)
}
