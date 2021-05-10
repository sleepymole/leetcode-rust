#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let (mut a, mut b) = (1, 1);
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                b = a + 1;
            } else if nums[i] < nums[i - 1] {
                a = b + 1;
            }
        }
        a.max(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wiggle_max_length() {
        assert_eq!(Solution::wiggle_max_length(vec![1, 7, 4, 9, 2, 5]), 6);
        assert_eq!(
            Solution::wiggle_max_length(vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8]),
            7
        );
        assert_eq!(
            Solution::wiggle_max_length(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
            2
        );
    }
}
