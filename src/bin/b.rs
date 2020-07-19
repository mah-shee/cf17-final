#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::cmp::{max, min};
#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let mut count_a = 0;
    let mut count_b = 0;
    let mut count_c = 0;
    for i in s.iter() {
        match i {
            'a' => count_a += 1,
            'b' => count_b += 1,
            'c' => count_c += 1,
            _ => (),
        }
    }
    if max(max(count_a, count_b), count_c) - min(min(count_a, count_b), count_c) <= 1 {
        println!("YES");
    } else {
        println!("NO");
    }
}
