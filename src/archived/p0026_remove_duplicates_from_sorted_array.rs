#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut j = 0;
        for i in 1..nums.len() {
            if nums[i] != nums[j] {
                j += 1;
                nums[j] = nums[i];
            }
        }
        nums.truncate(j + 1);
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![1, 1, 2];
        let n = Solution::remove_duplicates(&mut nums);
        assert_eq!(n, 2);
        assert_eq!(nums, vec![1, 2]);
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let n = Solution::remove_duplicates(&mut nums);
        assert_eq!(n, 5);
        assert_eq!(nums, vec![0, 1, 2, 3, 4]);
    }
}
