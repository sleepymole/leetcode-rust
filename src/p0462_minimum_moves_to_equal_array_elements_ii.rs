#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut nums = nums;
        nums.sort_unstable();
        let mut left_moves = vec![0; nums.len()];
        for i in 1..nums.len() {
            left_moves[i] = left_moves[i - 1] + (nums[i] - nums[i - 1]) as i64 * i as i64;
        }
        let mut mm = *left_moves.last().unwrap();
        let mut right_move = 0;
        for i in (0..nums.len() - 1).rev() {
            right_move += (nums.len() - 1 - i) as i64 * (nums[i + 1] - nums[i]) as i64;
            mm = mm.min(left_moves[i] + right_move)
        }
        mm as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_moves2() {
        assert_eq!(Solution::min_moves2(vec![1, 2, 3]), 2);
        assert_eq!(Solution::min_moves2(vec![1, 10, 2, 9]), 16);
    }
}
