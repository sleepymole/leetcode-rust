#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        for i in 1..nums.len() {
            if nums[i] < nums[i - 1] {
                return i as i32 - 1;
            }
        }
        nums.len() as i32 - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_peak_element() {
        let nums = vec![1, 2, 3, 1];
        let i = Solution::find_peak_element(nums.clone());
        assert!(i >= 0 && i < nums.len() as i32);
        let i = i as usize;
        assert!(
            (i == 0 || nums[i] > nums[i - 1]) && (i + 1 == nums.len() || nums[i] > nums[i + 1])
        );
        let nums = vec![1, 2, 1, 3, 5, 6, 4];
        let i = Solution::find_peak_element(nums.clone());
        assert!(i >= 0 && i < nums.len() as i32);
        let i = i as usize;
        assert!(
            (i == 0 || nums[i] > nums[i - 1]) && (i + 1 == nums.len() || nums[i] > nums[i + 1])
        );
    }
}
