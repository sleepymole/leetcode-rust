#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut sum = 0;
        let mut i = 0;
        while i < nums.len() {
            sum += nums[i];
            i += 2;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_pair_sum() {
        assert_eq!(Solution::array_pair_sum(vec![1, 4, 3, 2]), 4);
        assert_eq!(Solution::array_pair_sum(vec![6, 2, 6, 5, 1, 2]), 9);
    }
}
