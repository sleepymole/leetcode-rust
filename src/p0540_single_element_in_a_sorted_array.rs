#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |a, b| a ^ b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_non_duplicate() {
        assert_eq!(
            Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]),
            2
        );
        assert_eq!(
            Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]),
            10
        );
    }
}
