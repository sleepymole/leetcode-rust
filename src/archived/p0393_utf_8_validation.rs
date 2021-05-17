#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut iter = data.into_iter();
        while let Some(c) = iter.next() {
            let mut ones = 0;
            if c & 0b11111000 == 0b11110000 {
                ones = 4;
            } else if c & 0b11110000 == 0b11100000 {
                ones = 3;
            } else if c & 0b11100000 == 0b11000000 {
                ones = 2;
            } else if c & 0b10000000 > 0 {
                return false;
            }
            for _ in 1..ones {
                if let Some(c) = iter.next() {
                    if c & 0b11000000 != 0b10000000 {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_utf8() {
        assert!(Solution::valid_utf8(vec![197, 130, 1]));
        assert!(!Solution::valid_utf8(vec![235, 140, 4]));
    }
}
