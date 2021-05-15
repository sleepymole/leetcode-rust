#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut m = vec![0; 128];
        for c in s.chars().map(|c| c as usize) {
            m[c] += 1;
        }
        let mut has_odd = false;
        let mut longest = 0;
        for i in 0..128 {
            longest += m[i] >> 1 << 1;
            has_odd |= (m[i] & 1) > 0;
        }
        if has_odd {
            longest += 1;
        }
        longest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(Solution::longest_palindrome("abccccdd".to_owned()), 7);
        assert_eq!(Solution::longest_palindrome("a".to_owned()), 1);
        assert_eq!(Solution::longest_palindrome("bb".to_owned()), 2);
    }
}
