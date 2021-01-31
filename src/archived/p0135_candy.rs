#![allow(dead_code)]
pub struct Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut m = BTreeMap::new();
        for (i, v) in ratings.iter().enumerate() {
            m.entry(v).or_insert_with(Vec::new).push(i);
        }
        let mut sum = 0;
        let mut nums = vec![0; n];
        for iv in m.values() {
            for &i in iv {
                nums[i] = 1;
                if i > 0 && ratings[i - 1] < ratings[i] {
                    nums[i] = nums[i].max(nums[i - 1] + 1);
                }
                if i + 1 < n && ratings[i + 1] < ratings[i] {
                    nums[i] = nums[i].max(nums[i + 1] + 1);
                }
                sum += nums[i];
            }
        }
        sum as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candy() {
        assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
        assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
    }
}
