#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let mut m = Vec::new();
        m.resize(s.len() + 1, false);
        m[0] = true;
        for c in p.chars() {
            if c == '*' {
                for i in 0..m.len() {
                    if m[i] {
                        for j in i + 1..m.len() {
                            m[j] = true;
                        }
                        break;
                    }
                }
            } else {
                for i in (0..m.len() - 1).rev() {
                    m[i + 1] = m[i] && (c == '?' || s[i] == c);
                }
                m[0] = false;
            }
        }
        *m.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_match() {
        assert!(!Solution::is_match("aa".to_owned(), "a".to_owned()));
        assert!(Solution::is_match("aa".to_owned(), "*".to_owned()));
        assert!(!Solution::is_match("cb".to_owned(), "?a".to_owned()));
        assert!(Solution::is_match("adceb".to_owned(), "*a*b".to_owned()));
        assert!(!Solution::is_match("acdcb".to_owned(), "a*c?b".to_owned()));
    }
}
