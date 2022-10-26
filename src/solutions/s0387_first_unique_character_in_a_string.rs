#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut m = vec![0; 26];
        for c in s.chars() {
            m[c as usize - 'a' as usize] += 1;
        }
        for (i, c) in s.chars().enumerate() {
            if m[c as usize - 'a' as usize] == 1 {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_uniq_char() {
        assert_eq!(Solution::first_uniq_char("leetcode".to_owned()), 0);
        assert_eq!(Solution::first_uniq_char("loveleetcode".to_owned()), 2);
        assert_eq!(Solution::first_uniq_char("aabb".to_owned()), -1);
    }
}
