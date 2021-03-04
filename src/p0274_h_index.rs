#![allow(dead_code)]
pub struct Solution {}

use std::cmp::Reverse;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut citations = citations;
        citations.sort_unstable_by_key(|&v| Reverse(v));
        let mut hi = 0;
        for (i, &v) in citations.iter().enumerate() {
            if v >= (i + 1) as i32 {
                hi = (i + 1) as i32
            }
        }
        hi
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_h_index() {
        assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
    }
}
