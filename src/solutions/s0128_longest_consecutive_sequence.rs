#![allow(dead_code)]
pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut s = HashSet::new();
        for x in nums {
            s.insert(x);
        }
        let mut ans = 0;
        while !s.is_empty() {
            let x = *s.iter().next().unwrap();
            s.remove(&x);
            let mut n = 1;
            let mut l = x - 1;
            while s.remove(&l) {
                n += 1;
                l -= 1;
            }
            let mut r = x + 1;
            while s.remove(&r) {
                n += 1;
                r += 1;
            }
            ans = ans.max(n);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_consecutive() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }
}
