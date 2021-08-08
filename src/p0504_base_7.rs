#![allow(dead_code)]
pub struct Solution {}

use std::char;

impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        let sign = num.signum();
        let mut s = Vec::new();
        let mut num = num.abs();
        while num > 0 {
            s.push(char::from_u32((num % 7) as u32 + '0' as u32).unwrap());
            num /= 7;
        }
        let mut res = String::new();
        if sign < 0 {
            res.push('-');
        }
        for c in s.into_iter().rev() {
            res.push(c);
        }
        if res.is_empty() {
            res.push('0');
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_base7() {
        assert_eq!(Solution::convert_to_base7(100), "202".to_owned());
        assert_eq!(Solution::convert_to_base7(-7), "-10".to_owned());
    }
}
