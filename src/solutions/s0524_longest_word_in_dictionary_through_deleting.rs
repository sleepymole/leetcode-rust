#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    fn is_subsequence(a: &str, b: &str) -> bool {
        let mut b = b.chars();
        for c1 in a.chars() {
            let mut matched = false;
            for c2 in b.by_ref() {
                if c1 == c2 {
                    matched = true;
                    break;
                }
            }
            if !matched {
                return false;
            }
        }
        true
    }

    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        let mut dictionary = dictionary;
        dictionary.sort_unstable_by(|s1, s2| {
            if s1.len() != s2.len() {
                s2.len().cmp(&s1.len())
            } else {
                s1.cmp(s2)
            }
        });
        for w in dictionary {
            if Solution::is_subsequence(&w, &s) {
                return w;
            }
        }
        "".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_longest_word() {
        assert_eq!(
            Solution::find_longest_word(
                "abpcplea".to_owned(),
                vec![
                    "ale".to_owned(),
                    "apple".to_owned(),
                    "monkey".to_owned(),
                    "plea".to_owned()
                ]
            ),
            "apple".to_owned()
        );
        assert_eq!(
            Solution::find_longest_word(
                "abpcplea".to_owned(),
                vec!["a".to_owned(), "b".to_owned(), "c".to_owned()]
            ),
            "a".to_owned()
        );
    }
}
