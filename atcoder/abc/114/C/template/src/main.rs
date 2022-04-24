use proconio::input;
use std::cmp;
use std::collections::BTreeMap as map;
use std::collections::BTreeSet as set;
use std::collections::BinaryHeap as b_heap;
use std::collections::VecDeque as deque;

fn resolve(add: i64, digit: i64, bs: i64, limit: i64) -> i64 {
    let mut count = 0;
    if add + digit * 3 <= limit {
        count += resolve(add + digit * 3, digit * 10, bs | 1, limit);
        if (bs | 1) == 7 {
            count += 1
        }
    }
    if add + digit * 5 <= limit {
        count += resolve(add + digit * 5, digit * 10, bs | 2, limit);
        if (bs | 2) == 7 {
            count += 1
        }
    }
    if add + digit * 7 <= limit {
        count += resolve(add + digit * 7, digit * 10, bs | 4, limit);
        if (bs | 4) == 7 {
            count += 1
        }
    }
    return count;
}

fn main() {
    input! {
        n: i64,
    }

    println!("{}", resolve(0, 1, 0, n))
}
