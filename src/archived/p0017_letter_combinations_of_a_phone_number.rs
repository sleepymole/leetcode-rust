#![allow(dead_code)]
pub struct Solution;

use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }
        let m: HashMap<char, Vec<char>> = [
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z']),
        ]
        .iter()
        .cloned()
        .collect();
        let mut q: VecDeque<String> = VecDeque::new();
        for d in digits.chars() {
            let qlen = q.len();
            if qlen == 0 {
                for c in m.get(&d).unwrap() {
                    q.push_back(c.to_string());
                }
            } else {
                for _ in 0..qlen {
                    let s = q.pop_front().unwrap();
                    for c in m.get(&d).unwrap() {
                        let mut ss = s.clone();
                        ss.push(*c);
                        q.push_back(ss);
                    }
                }
            }
        }
        let mut ans: Vec<String> = Vec::new();
        for s in q.iter() {
            ans.push((*s).clone())
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_combinations() {
        let mut ans = Solution::letter_combinations("23".to_owned());
        ans.sort();
        assert_eq!(
            ans,
            vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
        assert!(Solution::letter_combinations("".to_owned()).is_empty());
        ans = Solution::letter_combinations("2".to_owned());
        ans.sort();
        assert_eq!(ans, vec!["a", "b", "c"]);
    }
}
