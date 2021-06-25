#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let a = minutes_to_test / minutes_to_die + 1;
        let mut pigs = 0;
        let mut state = 1;
        while state < buckets {
            state *= a;
            pigs += 1;
        }
        pigs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poor_pigs() {
        assert_eq!(Solution::poor_pigs(1000, 15, 60), 5);
        assert_eq!(Solution::poor_pigs(4, 15, 15), 2);
        assert_eq!(Solution::poor_pigs(4, 15, 30), 2);
    }
}
