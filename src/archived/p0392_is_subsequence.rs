#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut t = t.chars();
        for c1 in s.chars() {
            let mut found = false;
            while let Some(c2) = t.next() {
                if c1 == c2 {
                    found = true;
                    break;
                }
            }
            if !found {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_subsequence() {
        assert_eq!(
            Solution::is_subsequence("abc".to_owned(), "ahbgdc".to_owned()),
            true
        );
        assert_eq!(
            Solution::is_subsequence("axc".to_owned(), "ahbgdc".to_owned()),
            false
        );
    }
}
