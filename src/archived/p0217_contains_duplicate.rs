#![allow(dead_code)]
pub struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut m = HashSet::new();
        for x in nums {
            if m.contains(&x) {
                return true;
            }
            m.insert(x);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
        assert!(!Solution::contains_duplicate(vec![1, 2, 3, 4]));
        assert!(
            Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
    }
}
