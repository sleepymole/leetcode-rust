#![allow(dead_code)]
pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut m = HashMap::new();
        let mut sum = 0;
        m.insert(sum, 1);
        let mut count = 0;
        for x in nums {
            sum += x;
            if let Some(&v) = m.get(&(sum - k)) {
                count += v;
            }
            *m.entry(sum).or_insert(0) += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
    }
}
