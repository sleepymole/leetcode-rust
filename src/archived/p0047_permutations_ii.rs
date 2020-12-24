#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) -> bool {
        if nums.len() <= 1 {
            return false;
        }
        let mut i = nums.len() - 1;
        while i > 0 && nums[i - 1] >= nums[i] {
            i -= 1;
        }
        if i == 0 {
            return false;
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
        true
    }

    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let mut res = Vec::new();
        loop {
            res.push(nums.clone());
            if !Solution::next_permutation(&mut nums) {
                break;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute_unique() {
        assert_eq!(
            Solution::permute_unique(vec![1, 1, 2]),
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]
        );
        assert_eq!(
            Solution::permute_unique(vec![1, 2, 3]),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }
}
