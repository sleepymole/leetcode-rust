#![allow(dead_code)]
pub struct Solution {}

use std::i32;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut ans = i32::MAX;
        let mut sum = 0;
        let (mut left, mut right) = (0, 0);
        while right < nums.len() {
            while sum < target && right < nums.len() {
                sum += nums[right];
                right += 1;
            }
            while sum >= target && left < right {
                ans = ans.min((right - left) as i32);
                sum -= nums[left];
                left += 1;
            }
        }
        if ans == i32::MAX {
            ans = 0;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_sub_array_len() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
            0
        );
    }
}
