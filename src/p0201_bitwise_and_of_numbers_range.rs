#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        for i in (0..32).rev() {
            if m & (1 << i) != n & (1 << i) {
                let mask = (1 << (i + 1)) - 1;
                return (m | mask) ^ mask;
            }
        }
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_bitwise_and() {
        assert_eq!(Solution::range_bitwise_and(5, 7), 4);
        assert_eq!(Solution::range_bitwise_and(0, 1), 0);
    }
}
