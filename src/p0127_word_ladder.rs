#![allow(dead_code)]
pub struct Solution;

use std::collections::VecDeque;

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

    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let end_pos = word_list.iter().position(|x| x == &end_word);
        if end_pos.is_none() {
            return 0;
        }
        let end_pos = end_pos.unwrap();
        let n = word_list.len();
        let mut q = VecDeque::new();
        let mut vis = vec![false; n];
        for i in 0..n {
            if Solution::diff(&begin_word, &word_list[i]) == 1 {
                if i == end_pos {
                    return 2;
                }
                q.push_back(i);
                vis[i] = true;
            }
        }
        let mut steps = 2;
        while !q.is_empty() {
            let qlen = q.len();
            for _ in 0..qlen {
                let x = q.pop_front().unwrap();
                for i in 0..n {
                    if vis[i] || Solution::diff(&word_list[x], &word_list[i]) != 1 {
                        continue;
                    }
                    if i == end_pos {
                        return steps + 1;
                    }
                    q.push_back(i);
                    vis[i] = true;
                }
            }
            steps += 1;
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ladder_length() {
        assert_eq!(
            Solution::ladder_length(
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
            5
        );
        assert_eq!(
            Solution::ladder_length(
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
            0
        );
    }
}
