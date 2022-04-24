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

    let mut n_divs = vec![0; 105];
    let mut ans = 0i32;

    for val in 2..(n + 1) {
        let mut buf = val;
        for div in 2..105 {
            while buf % div == 0 {
                buf /= div;
                n_divs[div as usize] += 1;
            }
            if buf == 1 {
                continue;
            }
        }
    }

    // A^74
    for i in 0..(n_divs.len()) {
        if n_divs[i] >= 74 {
            ans += 1;
        }
    }

    // A^24 * B^2
    for i in 0..(n_divs.len()) {
        if n_divs[i] >= 24 {
            for j in 0..(n_divs.len()) {
                if n_divs[j] >= 2 && i != j {
                    ans += 1;
                }
            }
        }
    }

    // A^14 * B*4
    for i in 0..(n_divs.len()) {
        if n_divs[i] >= 14 {
            for j in 0..(n_divs.len()) {
                if n_divs[j] >= 4 && i != j {
                    ans += 1;
                }
            }
        }
    }

    // A^4 * B^4 * C^2
    for i in 0..(n_divs.len()) {
        if n_divs[i] >= 4 {
            for j in (i + 1)..(n_divs.len()) {
                if n_divs[j] >= 4 {
                    for k in 0..(n_divs.len()) {
                        if n_divs[k] >= 2 && k != j && k != i {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans)
}
