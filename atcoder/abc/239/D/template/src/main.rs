use proconio::input;
use std::cmp;
use std::collections::BTreeMap as map;
use std::collections::BTreeSet as set;
use std::collections::BinaryHeap as b_heap;
use std::collections::VecDeque as deque;

fn main() {
    let mut prime: [i32; 205] = [0; 205];
    for i in 2..201 {
        if prime[i] == 0 {
            prime[i] = 1;
            for j in 2..100 {
                if i * j > 201 {
                    break;
                } else if prime[i * j] == 0 {
                    prime[i * j] = -1;
                }
            }
        }
    }

    input! {
        a: i32,
        b: i32,
        c: i32,
        d: i32,
    }

    for i in a..=b {
        let mut takahashi = true;
        for j in c..=d {
            if prime[(i + j) as usize] == 1 {
                takahashi = false;
            }
        }

        if !takahashi {
            println!("Takahashi");
            return;
        }
    }

    println!("Aoki");
    return;
}
