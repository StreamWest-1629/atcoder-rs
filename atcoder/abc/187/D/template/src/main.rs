use proconio::input;
use std::cmp;
use std::collections::BTreeMap as map;
use std::collections::BTreeSet as set;
use std::collections::BinaryHeap as b_heap;
use std::collections::VecDeque as deque;

fn main() {
    input! {
        n: i32,
        ab: [(i64, i64); n],
    }

    let mut diff: Vec<i64> = ab.iter().map(|(a, b)| 2 * a + b).collect();
    diff.sort_by(|l, r| r.cmp(l));
    let sorted = diff;

    let mut sum_a: i64 = 0;
    for (a, _) in ab {
        sum_a += a;
    }

    let mut ans = 0;
    for larger in sorted {
        ans += 1;
        sum_a -= larger;
        if sum_a < 0 {
            println!("{}", ans);
            return;
        }
    }
}
