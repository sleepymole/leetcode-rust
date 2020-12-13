#![allow(dead_code)]
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut m: HashMap<char, i32> = HashMap::new();
        let (mut start, mut max_len) = (0, 0);
        for (i, c) in s.chars().enumerate() {
            if let Some(&p) = m.get(&c) {
                if p >= start {
                    start = p + 1;
                }
            }
            if i as i32 - start + 1 > max_len {
                max_len = i as i32 - start + 1;
            }
            m.insert(c, i as i32);
        }
        max_len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(
            Solution::length_of_longest_substring(String::from("abcabcbb")),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("bbbbb")),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring(String::from("pwwkew")),
            3
        );
    }
}
