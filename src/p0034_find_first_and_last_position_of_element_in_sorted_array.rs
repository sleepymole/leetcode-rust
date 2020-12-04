#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut l, mut r) = (0, nums.len());
        while l < r {
            let m = (l + r) / 2;
            if nums[m] < target {
                l = m + 1;
            } else {
                r = m;
            }
        }
        if l == nums.len() || nums[l] != target {
            return vec![-1, -1];
        }
        let s = l;
        r = nums.len();
        while l < r {
            let m = (l + r) / 2;
            if nums[m] <= target {
                l = m + 1;
            } else {
                r = m;
            }
        }
        vec![s as i32, (l - 1) as i32]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_range() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
            vec![3, 4]
        );
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
        assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
    }
}
