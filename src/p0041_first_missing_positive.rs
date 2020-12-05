#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        for i in 0..nums.len() {
            let mut x = nums[i];
            while x >= 1
                && x <= nums.len() as i32
                && x != (i + 1) as i32
                && nums[(x - 1) as usize] != x
            {
                let t = nums[(x - 1) as usize];
                nums[(x - 1) as usize] = x;
                x = t;
            }
            nums[i] = x;
        }
        for i in 0..nums.len() {
            if nums[i] != (i + 1) as i32 {
                return (i + 1) as i32;
            }
        }
        (nums.len() + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_missing_positive() {
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 13]), 1);
    }
}
