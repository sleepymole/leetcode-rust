#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();
        let mut m: Vec<bool> = Vec::with_capacity(s.len() + 1);
        m.resize(s.len() + 1, false);
        m[0] = true;
        for i in 0..p.len() {
            if i + 1 < p.len() && p[i + 1] == '*' {
                continue;
            }
            if p[i] == '.' {
                for j in (1..m.len()).rev() {
                    m[j] = m[j - 1];
                }
                m[0] = false;
            } else if p[i] == '*' {
                if p[i - 1] == '.' {
                    for j in 0..m.len() {
                        if m[j] {
                            for k in j..m.len() {
                                m[k] = true;
                            }
                            break;
                        }
                    }
                } else {
                    for j in 1..m.len() {
                        if s[j - 1] == p[i - 1] {
                            m[j] = m[j - 1] || m[j];
                        }
                    }
                }
            } else {
                for j in (1..m.len()).rev() {
                    m[j] = m[j - 1] && s[j - 1] == p[i];
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
        assert!(Solution::is_match("aa".to_owned(), "a*".to_owned()));
        assert!(Solution::is_match("ab".to_owned(), ".*".to_owned()));
        assert!(
            Solution::is_match("aab".to_owned(), "c*a*b".to_owned()));
        assert!(!
            Solution::is_match("mississippi".to_owned(), "mis*is*p*.".to_owned()));
    }
}
