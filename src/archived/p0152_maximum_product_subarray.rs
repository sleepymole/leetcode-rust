#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        let mut i = 0;
        while i < nums.len() {
            ans = ans.max(nums[i]);
            if nums[i] == 0 {
                i += 1;
                continue;
            }
            let mut product = nums[i];
            let mut j = i + 1;
            while j < nums.len() && nums[j] != 0 {
                product *= nums[j];
                ans = ans.max(product);
                j += 1;
            }
            let mut product = 1;
            for k in (i..j).rev() {
                product *= nums[k];
                ans = ans.max(product);
            }
            i = j;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_max_product() {
        assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
        assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
    }
}
