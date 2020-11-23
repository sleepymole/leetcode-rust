#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut ans: Option<i32> = None;
        for i in 0..nums.len() {
            let (mut l, mut r) = (i + 1, nums.len() - 1);
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if ans.is_none() || (sum - target).abs() < (ans.unwrap() - target).abs() {
                    ans = Some(sum);
                }
                if sum > target {
                    r -= 1;
                } else {
                    l += 1;
                }
            }
        }
        ans.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum_closest() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }
}
