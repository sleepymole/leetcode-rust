#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut nums = nums;
        nums.resize(n * 2, 0);
        for i in n..2 * n {
            nums[i] = nums[i - n];
        }
        let mut stk = Vec::new();
        let mut res = vec![0; n];
        for i in (0..2 * n).rev() {
            while !stk.is_empty() && stk[stk.len() - 1] <= nums[i] {
                stk.pop();
            }
            if i < n {
                res[i] = stk.last().cloned().unwrap_or(-1);
            }
            stk.push(nums[i]);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_greater_elements() {
        assert_eq!(
            Solution::next_greater_elements(vec![1, 2, 1]),
            vec![2, -1, 2]
        );
        assert_eq!(
            Solution::next_greater_elements(vec![1, 2, 3, 4, 3]),
            vec![2, 3, 4, -1, 4]
        );
    }
}
