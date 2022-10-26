#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() <= 1 {
            return;
        }
        let mut i = nums.len() - 1;
        while i > 0 && nums[i - 1] >= nums[i] {
            i -= 1;
        }
        if i == 0 {
            nums.sort_unstable();
            return;
        }
        for j in (i..nums.len()).rev() {
            if nums[j] > nums[i - 1] {
                nums.swap(j, i - 1);
                break;
            }
        }
        let mut j = nums.len() - 1;
        while i < j {
            nums.swap(j, i);
            i += 1;
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_permutation() {
        let mut nums = vec![1, 2, 3];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);
        let mut nums = vec![3, 2, 1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
        let mut nums = vec![1, 1, 5];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 5, 1]);
        let mut nums = vec![1];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![1]);
        let mut nums = vec![1, 3, 2];
        Solution::next_permutation(&mut nums);
        assert_eq!(nums, vec![2, 1, 3]);
    }
}
