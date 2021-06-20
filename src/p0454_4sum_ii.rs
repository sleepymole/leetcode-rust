#![allow(dead_code)]
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        let n = nums1.len();
        let mut m = HashMap::new();
        for i in 0..n {
            for j in 0..n {
                let v = nums1[i] + nums2[j];
                *m.entry(v).or_insert(0) += 1;
            }
        }
        let mut count = 0;
        for i in 0..n {
            for j in 0..n {
                let v = nums3[i] + nums4[j];
                if let Some(&x) = m.get(&-v) {
                    count += x;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_four_sum_count() {
        assert_eq!(
            Solution::four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
            2
        );
        assert_eq!(
            Solution::four_sum_count(vec![0], vec![0], vec![0], vec![0]),
            1
        );
    }
}
