#![allow(dead_code)]
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ms = vec![HashMap::new(); n];
        let mut res = 0;
        for i in 1..nums.len() {
            for j in 0..i {
                let diff = nums[i] as i64 - nums[j] as i64;
                let mut count = 1;
                if let Some(&v) = ms[j].get(&diff) {
                    res += v;
                    count += v;
                }
                *ms[i].entry(diff).or_insert(0) += count;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_arithmetic_slices() {
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![2, 4, 6, 8, 10]),
            7
        );
        assert_eq!(
            Solution::number_of_arithmetic_slices(vec![7, 7, 7, 7, 7]),
            16
        );
    }
}
