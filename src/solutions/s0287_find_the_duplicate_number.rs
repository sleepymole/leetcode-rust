#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        while nums[0] != nums[nums[0] as usize] {
            let i = nums[0] as usize;
            nums.swap(0, i);
        }
        nums[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicate() {
        assert_eq!(Solution::find_duplicate(vec![1, 3, 4, 2, 2]), 2);
        assert_eq!(Solution::find_duplicate(vec![3, 1, 3, 4, 2]), 3);
        assert_eq!(Solution::find_duplicate(vec![1, 1]), 1);
        assert_eq!(Solution::find_duplicate(vec![1, 1, 2]), 1);
    }
}
