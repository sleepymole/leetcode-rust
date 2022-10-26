#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let (mut l, mut r) = (0, nums.len() - 1);
        while l + 1 < r {
            let m = (l + r) / 2;
            if nums[m] > nums[0] {
                l = m;
            } else {
                r = m;
            }
        }
        if nums[l] < nums[r] {
            l += 1;
        }
        l += 1;
        r = l + nums.len();
        while l < r {
            let m = (l + r) / 2;
            if nums[m % nums.len()] < target {
                l = m + 1;
            } else {
                r = m;
            }
        }
        if nums[l % nums.len()] != target {
            return -1;
        }
        (l % nums.len()) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(Solution::search(vec![1], 0), -1);
    }
}
