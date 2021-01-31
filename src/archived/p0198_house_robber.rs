#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut f = vec![0; nums.len()];
        let mut ans = 0;
        for i in 0..nums.len() {
            let mut x = 0;
            if i >= 2 {
                x = f[i - 2];
            }
            if i >= 3 {
                x = x.max(f[i - 3]);
            }
            f[i] = nums[i] + x;
            ans = ans.max(f[i]);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob() {
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    }
}
