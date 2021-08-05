#![allow(dead_code)]
pub struct Solution;

use std::collections::BTreeSet;

impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        if nums.len() < 3 {
            return false;
        }
        let mut f = vec![0; nums.len()];
        f[0] = nums[0];
        for i in 1..nums.len() {
            f[i] = f[i - 1].min(nums[i]);
        }
        let mut s = BTreeSet::new();
        for i in (1..nums.len()).rev() {
            if let Some(&v) = s.range(..nums[i]).rev().next() {
                if f[i - 1] < nums[i] && v > f[i - 1] {
                    return true;
                }
            }
            s.insert(nums[i]);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find132pattern() {
        assert!(!Solution::find132pattern(vec![1, 2, 3, 4]));
        assert!(Solution::find132pattern(vec![3, 1, 4, 2]));
        assert!(Solution::find132pattern(vec![-1, 3, 2, 0]));
    }
}
