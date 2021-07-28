#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut ways = 0;
        for i in 0..(1 << n) {
            let mut sum = 0;
            for j in 0..n {
                sum += if i & (1 << j) > 0 { nums[j] } else { -nums[j] };
            }
            if sum == target {
                ways += 1;
            }
        }
        ways
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_target_sum_ways() {
        assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
        assert_eq!(Solution::find_target_sum_ways(vec![1], 1), 1);
    }
}
