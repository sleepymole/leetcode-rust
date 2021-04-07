#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        if s.is_empty() {
            return s;
        }
        let mut s: Vec<char> = s.chars().collect();
        let is_vowel = |c: char| {
            let c = c.to_ascii_lowercase();
            c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
        };
        let (mut i, mut j) = (0, s.len() - 1);
        while i < j {
            while i < j && !is_vowel(s[i]) {
                i += 1;
            }
            while i < j && !is_vowel(s[j]) {
                j -= 1;
            }
            if i < j {
                s.swap(i, j);
                i += 1;
                j -= 1;
            }
        }
        s.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_vowels() {
        assert_eq!(
            Solution::reverse_vowels("hello".to_owned()),
            "holle".to_owned()
        );
        assert_eq!(
            Solution::reverse_vowels("leetcode".to_owned()),
            "leotcede".to_owned()
        );
    }
}
