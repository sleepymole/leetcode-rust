#![allow(dead_code)]
pub struct Solution;

use std::char;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut res = 0;
        for i in 'A' as u32..='Z' as u32 {
            let c = char::from_u32(i).unwrap();
            let (mut i, mut j) = (0, 0);
            let mut intr = 0;
            while j < chars.len() {
                while j < chars.len() && (chars[j] == c || intr < k) {
                    if chars[j] != c {
                        intr += 1;
                    }
                    j += 1;
                }
                res = res.max(j - i);
                if i < j {
                    if chars[i] != c {
                        intr -= 1;
                    }
                    i += 1;
                } else {
                    i += 1;
                    j += 1;
                }
            }
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_replacement() {
        assert_eq!(Solution::character_replacement("ABAB".to_owned(), 2), 4);
        assert_eq!(Solution::character_replacement("AABABBA".to_owned(), 1), 4);
    }
}
