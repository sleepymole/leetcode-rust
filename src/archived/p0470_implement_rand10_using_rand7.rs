#![allow(dead_code)]
pub struct Solution;

fn rand7() -> i32 {
    0
}

impl Solution {
    pub fn rand10() -> i32 {
        let x = (rand7() - 1) * 7 + rand7() - 1;
        if x < 40 {
            x % 10 + 1
        } else {
            Solution::rand10()
        }
    }
}
