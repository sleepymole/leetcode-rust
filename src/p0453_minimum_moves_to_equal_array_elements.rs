#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        nums.iter().sum::<i32>() - nums.len() as i32 * nums.iter().min().cloned().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_moves() {
        assert_eq!(Solution::min_moves(vec![1, 2, 3]), 3);
        assert_eq!(Solution::min_moves(vec![1, 1, 1]), 0);
    }
}
