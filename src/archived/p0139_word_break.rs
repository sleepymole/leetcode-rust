#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let s = s.into_bytes();
        let word_dict: Vec<Vec<u8>> = word_dict.into_iter().map(|x| x.into_bytes()).collect();
        let mut f = vec![false; s.len() + 1];
        f[0] = true;
        for i in 0..s.len() {
            if !f[i] {
                continue;
            }
            for w in &word_dict {
                if w.len() <= s.len() - i && !f[i + w.len()] {
                    let mut matched = true;
                    for j in 0..w.len() {
                        if s[i + j] != w[j] {
                            matched = false;
                            break;
                        }
                    }
                    if matched {
                        f[i + w.len()] = true;
                    }
                }
            }
        }
        f[s.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_break() {
        assert!(
            Solution::word_break(
                "leetcode".to_owned(),
                vec!["leet".to_owned(), "code".to_owned()]
            ));
        assert!(
            Solution::word_break(
                "applepenapple".to_owned(),
                vec!["apple".to_owned(), "pen".to_owned()]
            ));
        assert!(!
            Solution::word_break(
                "catsandog".to_owned(),
                vec![
                    "cats".to_owned(),
                    "dog".to_owned(),
                    "sand".to_owned(),
                    "and".to_owned(),
                    "cat".to_owned()
                ]
            ));
    }
}
