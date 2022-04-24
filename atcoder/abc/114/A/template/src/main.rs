use proconio::input;
use std::cmp;
use std::collections::BTreeMap as map;
use std::collections::BTreeSet as set;
use std::collections::BinaryHeap as b_heap;
use std::collections::VecDeque as deque;

fn main() {
    input! {
        age: i32,
    }

    if age == 3 || age == 5 || age == 7 {
        println!("YES")
    } else {
        println!("NO")
    }
}
