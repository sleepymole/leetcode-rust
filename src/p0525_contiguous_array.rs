#![allow(dead_code)]
pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut m = HashMap::new();
        let mut sum = 0;
        m.insert(sum, -1);
        let mut res = 0;
        for i in 0..nums.len() {
            sum += if nums[i] == 1 { 1 } else { -1 };
            if let Some(&j) = m.get(&sum) {
                res = res.max(i as i32 - j);
            }
            m.entry(sum).or_insert(i as i32);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_length() {
        assert_eq!(Solution::find_max_length(vec![0, 1]), 2);
        assert_eq!(Solution::find_max_length(vec![0, 1, 0]), 2);
    }
}
