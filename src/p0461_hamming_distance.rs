#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut d = 0;
        let mut xor = x ^ y;
        while xor > 0 {
            d += xor & 1;
            xor >>= 1;
        }
        d
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_distance() {
        assert_eq!(Solution::hamming_distance(1, 4), 2);
        assert_eq!(Solution::hamming_distance(3, 1), 1);
    }
}
