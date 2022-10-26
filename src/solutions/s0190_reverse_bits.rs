#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut x = x;
        let mut ans = 0;
        for _ in 0..32 {
            ans = ans << 1 | x & 1;
            x >>= 1;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_bits() {
        assert_eq!(Solution::reverse_bits(43261596), 964176192);
        assert_eq!(Solution::reverse_bits(4294967293), 3221225471);
    }
}
