#![allow(dead_code)]
pub struct Solution;

use std::char;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut x = 0;
        for c in s.chars() {
            x ^= c as u8;
        }
        for c in t.chars() {
            x ^= c as u8;
        }
        char::from_u32(x as u32).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_the_difference() {
        assert_eq!(
            Solution::find_the_difference("abcd".to_owned(), "abcde".to_owned()),
            'e'
        );
        assert_eq!(
            Solution::find_the_difference("".to_owned(), "y".to_owned()),
            'y'
        );
        assert_eq!(
            Solution::find_the_difference("a".to_owned(), "aa".to_owned()),
            'a'
        );
        assert_eq!(
            Solution::find_the_difference("ae".to_owned(), "aea".to_owned()),
            'a'
        );
    }
}
