#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut k = 0;
        let mut ans = 0;
        let mut i = 0;
        while i < nums.len() {
            if nums[i] as i64 <= k {
                k += nums[i] as i64;
                i += 1;
            } else if nums[i] as i64 == k + 1 {
                k = 2 * k + 1;
                i += 1;
            } else {
                k = 2 * k + 1;
                ans += 1;
            }
            if k >= n as i64 {
                return ans;
            }
        }
        while k < n as i64 {
            k = 2 * k + 1;
            ans += 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_patches() {
        assert_eq!(Solution::min_patches(vec![1, 3], 6), 1);
        assert_eq!(Solution::min_patches(vec![1, 5, 10], 20), 2);
        assert_eq!(Solution::min_patches(vec![1, 2, 2], 5), 0);
    }
}
