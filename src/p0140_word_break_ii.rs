#![allow(dead_code)]
pub struct Solution;

use std::{str, usize};

impl Solution {
    fn construct(origin: &[u8], last_index: usize, table: &[Vec<usize>]) -> Vec<String> {
        if last_index == 0 {
            return vec!["".to_owned()];
        }
        let mut ans = Vec::new();
        let prev_indexes = &table[last_index];
        for &prev_index in prev_indexes {
            for mut s in Solution::construct(origin, prev_index, table) {
                if !s.is_empty() {
                    s.push(' ');
                }
                s.push_str(str::from_utf8(&origin[prev_index..last_index]).unwrap());
                ans.push(s);
            }
        }
        ans
    }

    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let s = s.into_bytes();
        let word_dict: Vec<Vec<u8>> = word_dict.into_iter().map(|x| x.into_bytes()).collect();
        let mut f = vec![vec![]; s.len() + 1];
        f[0] = vec![usize::MAX];
        for i in 0..s.len() {
            if f[i].is_empty() {
                continue;
            }
            for w in &word_dict {
                if w.len() <= s.len() - i {
                    let mut matched = true;
                    for j in 0..w.len() {
                        if s[i + j] != w[j] {
                            matched = false;
                            break;
                        }
                    }
                    if matched {
                        f[i + w.len()].push(i);
                    }
                }
            }
        }
        Solution::construct(&s, s.len(), &f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut ss: Vec<String>) -> Vec<String> {
        ss.sort();
        ss
    }

    #[test]
    fn test_word_break() {
        assert_eq!(
            sorted(Solution::word_break(
                "catsanddog".to_owned(),
                vec![
                    "cat".to_owned(),
                    "cats".to_owned(),
                    "and".to_owned(),
                    "sand".to_owned(),
                    "dog".to_owned()
                ]
            )),
            sorted(vec!["cats and dog".to_owned(), "cat sand dog".to_owned()])
        );
        assert_eq!(
            sorted(Solution::word_break(
                "pineapplepenapple".to_owned(),
                vec![
                    "apple".to_owned(),
                    "pen".to_owned(),
                    "applepen".to_owned(),
                    "pine".to_owned(),
                    "pineapple".to_owned()
                ]
            )),
            sorted(vec![
                "pine apple pen apple".to_owned(),
                "pineapple pen apple".to_owned(),
                "pine applepen apple".to_owned()
            ])
        );
        assert_eq!(
            Solution::word_break(
                "catsandog".to_owned(),
                vec![
                    "cats".to_owned(),
                    "dog".to_owned(),
                    "sand".to_owned(),
                    "and".to_owned(),
                    "cat".to_owned()
                ]
            ),
            Vec::<String>::new()
        );
    }
}
