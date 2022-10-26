#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut b = 1;
        while (num & !b) != 0 {
            b = (b << 1) + 1;
        }
        num ^ b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_complement() {
        assert_eq!(Solution::find_complement(5), 2);
        assert_eq!(Solution::find_complement(1), 0);
    }
}
