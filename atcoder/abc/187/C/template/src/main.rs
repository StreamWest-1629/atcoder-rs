use proconio::input;
use std::cmp;
use std::collections::BTreeMap as map;
use std::collections::BTreeSet as set;
use std::collections::BinaryHeap as b_heap;
use std::collections::VecDeque as deque;

fn main() {
    input! {
        n: i32,
        strs: [String; n],
    }

    let mut mp = map::<String, i32>::new();
    let mut ans = String::from("satisfiable");

    for s in strs {
        let check = s.as_bytes()[0 as usize] == ('!' as u8);
        let key: String = if check { (&s[1..]).to_string() } else { s };
        let val_check: i32 = if check { 1 } else { 2 };
        let val_set: i32 = if check { 2 } else { 1 };

        let val = 0;
        match mp.get(&key) {
            Some(&val) => {
                if val == val_check {
                    ans = key;
                    break;
                } else {
                    mp.insert(key, val_set);
                }
            }
            _ => {
                mp.insert(key, val_set);
            }
        }
    }

    println!("{}", ans)
}
