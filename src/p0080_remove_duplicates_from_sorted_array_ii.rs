#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut i = 1;
        for j in 1..nums.len() {
            if i == 1 || nums[j] != nums[j - 1] || nums[i - 1] != nums[i - 2] {
                nums[i] = nums[j];
                i += 1;
            }
        }
        nums.truncate(i);
        i as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let n = Solution::remove_duplicates(&mut nums);
        assert_eq!(n, 5);
        assert_eq!(nums, vec![1, 1, 2, 2, 3]);
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let n = Solution::remove_duplicates(&mut nums);
        assert_eq!(n, 7);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 3, 3]);
    }
}
