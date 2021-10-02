#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut stk = Vec::new();
        let mut l = nums.len();
        for i in 0..nums.len() {
            while !stk.is_empty() && nums[i] < nums[*stk.last().unwrap()] {
                l = l.min(stk.pop().unwrap());
            }
            stk.push(i);
        }
        let mut r = 0;
        stk.clear();
        for i in (0..nums.len()).rev() {
            while !stk.is_empty() && nums[i] > nums[*stk.last().unwrap()] {
                r = r.max(stk.pop().unwrap());
            }
            stk.push(i);
        }
        if l <= r {
            (r - l + 1) as i32
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_unsorted_subarray() {
        assert_eq!(
            Solution::find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15]),
            5
        );
        assert_eq!(Solution::find_unsorted_subarray(vec![1, 2, 3, 4]), 0);
        assert_eq!(Solution::find_unsorted_subarray(vec![1]), 0);
    }
}
