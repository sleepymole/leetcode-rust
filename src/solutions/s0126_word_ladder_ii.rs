#![allow(dead_code)]
pub struct Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    fn diff(s1: &String, s2: &String) -> usize {
        let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
        if s1.len() != s2.len() {
            return s1.len().max(s2.len());
        }
        let mut diff = 0;
        for i in 0..s1.len() {
            if s1[i] != s2[i] {
                diff += 1;
            }
        }
        diff
    }

    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let end_pos = word_list.iter().position(|x| x == &end_word);
        if end_pos.is_none() {
            return vec![];
        }
        let end_pos = end_pos.unwrap();
        let n = word_list.len();
        let mut q = VecDeque::new();
        let mut vis = vec![false; n];
        let mut paths = vec![vec![]; n];
        for i in 0..n {
            if Solution::diff(&begin_word, &word_list[i]) == 1 {
                q.push_back(i);
                paths[i] = vec![vec![begin_word.clone(), word_list[i].clone()]];
                vis[i] = true;
            }
        }
        while !q.is_empty() {
            let qlen = q.len();
            let mut new_items = HashSet::new();
            for _ in 0..qlen {
                let x = q.pop_front().unwrap();
                for i in 0..n {
                    if vis[i] || Solution::diff(&word_list[x], &word_list[i]) != 1 {
                        continue;
                    }
                    for p in paths[x].clone() {
                        let mut p = p.clone();
                        p.push(word_list[i].clone());
                        paths[i].push(p);
                    }
                    new_items.insert(i);
                }
            }
            for &i in &new_items {
                q.push_back(i);
                vis[i] = true;
            }
        }
        paths[end_pos].sort_unstable();
        paths[end_pos].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_ladders() {
        assert_eq!(
            Solution::find_ladders(
                "hit".to_owned(),
                "cog".to_owned(),
                vec![
                    "hot".to_owned(),
                    "dot".to_owned(),
                    "dog".to_owned(),
                    "lot".to_owned(),
                    "log".to_owned(),
                    "cog".to_owned()
                ]
            ),
            vec![
                vec![
                    "hit".to_owned(),
                    "hot".to_owned(),
                    "dot".to_owned(),
                    "dog".to_owned(),
                    "cog".to_owned()
                ],
                vec![
                    "hit".to_owned(),
                    "hot".to_owned(),
                    "lot".to_owned(),
                    "log".to_owned(),
                    "cog".to_owned()
                ]
            ]
        );
        assert_eq!(
            Solution::find_ladders(
                "hit".to_owned(),
                "cog".to_owned(),
                vec![
                    "hot".to_owned(),
                    "dot".to_owned(),
                    "dog".to_owned(),
                    "lot".to_owned(),
                    "log".to_owned()
                ]
            ),
            Vec::<Vec<String>>::new()
        );
    }
}
