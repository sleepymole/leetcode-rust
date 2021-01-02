#![allow(dead_code)]
pub struct Solution;

use std::char;

impl Solution {
    pub fn convert_to_title(n: i32) -> String {
        let mut s = String::new();
        let mut n = n;
        while n > 0 {
            s.push(char::from_u32((n - 1) as u32 % 26 + 'A' as u32).unwrap());
            n = (n - 1) / 26;
        }
        s.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_title() {
        assert_eq!(Solution::convert_to_title(1), "A".to_owned());
        assert_eq!(Solution::convert_to_title(28), "AB".to_owned());
        assert_eq!(Solution::convert_to_title(701), "ZY".to_owned());
    }
}
