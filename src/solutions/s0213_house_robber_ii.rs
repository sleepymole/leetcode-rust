#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn line_rob(nums: &[i32]) -> i32 {
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

    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        Solution::line_rob(&nums[0..nums.len() - 1]).max(Solution::line_rob(&nums[1..nums.len()]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob() {
        assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
        assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
        assert_eq!(Solution::rob(vec![0]), 0);
    }
}
