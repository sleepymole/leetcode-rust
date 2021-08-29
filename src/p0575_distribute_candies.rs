#![allow(dead_code)]
pub struct Solution {}

use std::collections::HashSet;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        let mut s = HashSet::new();
        for &c in &candy_type {
            s.insert(c);
        }
        s.iter().count().min(candy_type.len() / 2) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distribute_candies() {
        assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 2, 3, 3]), 3);
        assert_eq!(Solution::distribute_candies(vec![1, 1, 2, 3]), 2);
        assert_eq!(Solution::distribute_candies(vec![6, 6, 6, 6]), 1);
    }
}
