#![allow(dead_code)]
pub struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut s = HashSet::new();
        let mut i = 0;
        let mut res = 0;
        while i < nums.len() {
            let mut j = i + 1;
            while j < nums.len() && nums[j] == nums[i] {
                j += 1;
            }
            if k == 0 {
                if j - i > 1 {
                    res += 1;
                }
            } else {
                if s.contains(&(nums[i] - k)) {
                    res += 1;
                }
                s.insert(nums[i]);
            }
            i = j;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_pairs() {
        assert_eq!(Solution::find_pairs(vec![3, 1, 4, 1, 5], 2), 2);
        assert_eq!(Solution::find_pairs(vec![1, 2, 3, 4, 5], 1), 4);
        assert_eq!(Solution::find_pairs(vec![1, 3, 1, 5, 4], 0), 1);
    }
}
