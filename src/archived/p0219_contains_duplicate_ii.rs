#![allow(dead_code)]
pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut m = HashMap::new();
        for (i, x) in nums.iter().enumerate() {
            if let Some(j) = m.get(x) {
                if i - j <= k as usize {
                    return true;
                }
            }
            m.insert(x, i);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_nearby_duplicate() {
        assert!(
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3));
        assert!(
            Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1));
        assert!(!
            Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2));
    }
}
