#![allow(dead_code)]
pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut m = HashMap::new();
        m.insert(0, -1);
        let mut sum = 0;
        for i in 0..nums.len() {
            sum = (sum + nums[i]) % k;
            if let Some(&j) = m.get(&sum) {
                if i as i32 > j + 1 {
                    return true;
                }
            }
            m.entry(sum).or_insert(i as i32);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_subarray_sum() {
        assert!(Solution::check_subarray_sum(vec![23, 2, 4, 6, 7], 6));
        assert!(Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 6));
        assert!(!Solution::check_subarray_sum(vec![23, 2, 6, 4, 7], 13));
    }
}
