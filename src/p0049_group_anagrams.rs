#![allow(dead_code)]
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut m = HashMap::new();
        for s in strs.into_iter() {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort();
            let key: String = chars.into_iter().collect();
            m.entry(key).or_insert(vec![]).push(s);
        }
        m.into_iter().map(|(_, v)| v).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(groups: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut groups = groups;
        for g in groups.iter_mut() {
            g.sort();
        }
        groups.sort();
        groups
    }

    #[test]
    fn test_group_anagrams() {
        assert_eq!(
            sorted(Solution::group_anagrams(vec![
                "eat".to_owned(),
                "tea".to_owned(),
                "tan".to_owned(),
                "ate".to_owned(),
                "nat".to_owned(),
                "bat".to_owned()
            ])),
            sorted(vec![
                vec!["bat".to_owned()],
                vec!["nat".to_owned(), "tan".to_owned()],
                vec!["ate".to_owned(), "eat".to_owned(), "tea".to_owned()]
            ])
        );
        assert_eq!(
            Solution::group_anagrams(vec!["".to_owned()]),
            vec![vec!["".to_owned()]]
        );
        assert_eq!(
            Solution::group_anagrams(vec!["a".to_owned()]),
            vec![vec!["a".to_owned()]]
        )
    }
}
