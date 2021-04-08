#![allow(dead_code)]
pub struct Solution;

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let s1: HashSet<i32> = HashSet::from_iter(nums1);
        let s2: HashSet<i32> = nums2.into_iter().filter(|x| s1.contains(&x)).collect();
        let mut ans: Vec<i32> = s2.into_iter().collect();
        ans.sort_unstable();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        assert_eq!(
            Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]),
            vec![2]
        );
        assert_eq!(
            Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
            vec![4, 9]
        );
    }
}
