#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let p: Vec<char> = pattern.chars().collect();
        let ss: Vec<&str> = s.split_whitespace().collect();
        if p.len() != ss.len() {
            return false;
        }
        for i in 0..p.len() {
            for j in i + 1..p.len() {
                if (p[i] == p[j]) != (ss[i] == ss[j]) {
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
    fn test_word_pattern() {
        assert!(Solution::word_pattern(
            "abba".to_owned(),
            "dog cat cat dog".to_owned()
        ));
        assert!(!Solution::word_pattern(
            "abba".to_owned(),
            "dog cat cat fish".to_owned()
        ));
        assert!(!Solution::word_pattern(
            "aaaa".to_owned(),
            "dog cat cat dog".to_owned()
        ));
        assert!(!Solution::word_pattern(
            "abba".to_owned(),
            "dog dog dog dog".to_owned()
        ));
    }
}
