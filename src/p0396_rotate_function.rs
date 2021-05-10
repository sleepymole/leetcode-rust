#![allow(dead_code)]
#![allow(clippy::suspicious_operation_groupings)]
pub struct Solution;

impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut last_res = 0;
        for (i, &x) in nums.iter().enumerate() {
            sum += x as i64;
            last_res += i as i64 * x as i64;
        }
        let mut max_res = last_res;
        for i in 1..nums.len() {
            let res = last_res - sum + nums.len() as i64 * nums[i - 1] as i64;
            max_res = max_res.max(res);
            last_res = res;
        }
        max_res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_rotate_function() {
        assert_eq!(Solution::max_rotate_function(vec![4, 3, 2, 6]), 26);
        assert_eq!(Solution::max_rotate_function(vec![1000000007]), 0);
    }
}
