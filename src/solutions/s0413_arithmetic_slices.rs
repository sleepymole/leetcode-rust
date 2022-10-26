#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut count = 0;
        while i + 2 < nums.len() {
            let mut j = i + 2;
            while j < nums.len() && nums[j] + nums[j - 2] == nums[j - 1] * 2 {
                count += (j - i - 1) as i32;
                j += 1;
            }
            i = j - 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_arithmetic_slices() {
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4]), 3);
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1]), 0);
    }
}
