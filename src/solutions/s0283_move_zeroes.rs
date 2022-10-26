#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut zeros = 0;
        for i in 0..nums.len() {
            if nums[i] == 0 {
                zeros += 1;
            } else if zeros > 0 {
                nums.swap(i, i - zeros);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_move_zeros() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }
}
