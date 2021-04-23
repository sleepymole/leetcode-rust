#![allow(dead_code)]
#![allow(non_snake_case)]
pub struct Solution;

unsafe fn guess(_: i32) -> i32 {
    0
}

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let (mut l, mut r) = (1, n as u32 + 1);
        while l < r {
            let m = l + (r - l) / 2;
            if guess(m as i32) <= 0 {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l as i32
    }
}
