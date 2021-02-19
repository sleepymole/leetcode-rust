#![allow(dead_code)]
pub struct Solution {}

use std::collections::BTreeSet;
use std::i32;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let k = k as usize;
        let mut s = BTreeSet::new();
        let mut j = 0;
        for i in 0..nums.len() {
            if i - j > k {
                s.remove(&nums[j]);
                j += 1;
            }
            if let Some(&v) = s.range(nums[i].max(i32::MIN + t) - t..).next() {
                if v <= nums[i].min(i32::MAX - t) + t {
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
    fn test_contains_nearby_almost_duplicate() {
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(vec![1, 2, 3, 1], 3, 0),
            true
        );
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(vec![1, 0, 1, 1], 1, 2),
            true
        );
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(vec![1, 5, 9, 1, 5, 9], 2, 3),
            false
        );
        assert_eq!(
            Solution::contains_nearby_almost_duplicate(
                vec![2147483647, -1, 2147483647],
                1,
                2147483647
            ),
            false
        )
    }
}
