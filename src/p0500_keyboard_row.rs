#![allow(dead_code)]
pub struct Solution;

use std::collections::HashSet;

impl Solution {
    fn make_set(s: &str) -> HashSet<char> {
        let mut set = HashSet::new();
        for c in s.chars() {
            set.insert(c);
        }
        set
    }

    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let mut res = Vec::new();
        let rows = vec![
            Solution::make_set("qwertyuiopQWERTYUIOP"),
            Solution::make_set("asdfghjklASDFGHJKL"),
            Solution::make_set("zxcvbnmZXCVBNM"),
        ];
        for w in words {
            for row in &rows {
                if w.chars().filter(|c| !row.contains(c)).count() == 0 {
                    res.push(w.clone());
                    break;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_words() {
        assert_eq!(
            Solution::find_words(vec![
                "Hello".to_owned(),
                "Alaska".to_owned(),
                "Dad".to_owned(),
                "Peace".to_owned()
            ]),
            vec!["Alaska".to_owned(), "Dad".to_owned()]
        );
        assert!(Solution::find_words(vec!["omk".to_owned()]).is_empty());
        assert_eq!(
            Solution::find_words(vec!["adsdf".to_owned(), "sfd".to_owned()]),
            vec!["adsdf".to_owned(), "sfd".to_owned()]
        );
    }
}
