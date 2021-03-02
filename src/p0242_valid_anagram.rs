#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s: Vec<char> = s.chars().collect();
        let mut t: Vec<char> = t.chars().collect();
        s.sort_unstable();
        t.sort_unstable();
        s == t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        assert_eq!(
            Solution::is_anagram("anagram".to_owned(), "nagaram".to_owned()),
            true
        );
        assert_eq!(
            Solution::is_anagram("rat".to_owned(), "car".to_owned()),
            false
        );
    }
}
