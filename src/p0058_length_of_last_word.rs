#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut n = 0;
        for ss in s.split_whitespace() {
            n = ss.len()
        }
        n as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_last_word() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_owned()), 5);
        assert_eq!(Solution::length_of_last_word(" ".to_owned()), 0);
    }
}
