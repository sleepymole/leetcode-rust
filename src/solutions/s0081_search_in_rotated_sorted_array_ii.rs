#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.is_empty() {
            return false;
        }
        let (mut l, mut r) = (0, nums.len() - 1);
        while l + 1 < r {
            let m = (l + r) / 2;
            if nums[l] == nums[m] {
                l += 1;
            } else if nums[l] < nums[m] {
                if nums[l] <= target && target <= nums[m] {
                    r = m;
                } else {
                    l = m + 1;
                }
            } else if nums[m] <= target && target <= nums[r] {
                l = m;
            } else {
                r = m - 1;
            }
        }
        nums[l] == target || nums[r] == target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        assert!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
        assert!(!Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
    }
}
