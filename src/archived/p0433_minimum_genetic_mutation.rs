#![allow(dead_code)]
pub struct Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    fn cast_to_i32(s: String) -> i32 {
        let mut v = 0;
        for c in s.chars() {
            v = (v << 2)
                + match c {
                    'A' => 0,
                    'G' => 1,
                    'C' => 2,
                    'T' => 3,
                    _ => unreachable!(),
                };
        }
        v
    }

    fn can_mutate(a: i32, b: i32) -> bool {
        let mut x = a ^ b;
        while x > 0 {
            if x & 0b11 > 0 && x >> 2 > 0 {
                return false;
            }
            x >>= 2;
        }
        true
    }

    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        let start = Solution::cast_to_i32(start);
        let end = Solution::cast_to_i32(end);
        if start == end {
            return 0;
        }
        let mut genes = HashSet::new();
        for g in bank {
            let gi = Solution::cast_to_i32(g);
            if gi != start {
                genes.insert(gi);
            }
        }
        if !genes.contains(&end) {
            return -1;
        }
        let mut q = VecDeque::new();
        q.push_back((start, 0));
        while let Some((u, step)) = q.pop_front() {
            let mut opts = Vec::new();
            for &v in &genes {
                if Solution::can_mutate(u, v) {
                    if v == end {
                        return step + 1;
                    }
                    opts.push(v);
                    q.push_back((v, step + 1));
                }
            }
            for v in opts {
                genes.remove(&v);
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_mutation() {
        assert_eq!(
            Solution::min_mutation(
                "AACCGGTT".to_owned(),
                "AACCGGTA".to_owned(),
                vec!["AACCGGTA".to_owned()]
            ),
            1
        );
        assert_eq!(
            Solution::min_mutation(
                "AACCGGTT".to_owned(),
                "AAACGGTA".to_owned(),
                vec![
                    "AACCGGTA".to_owned(),
                    "AACCGCTA".to_owned(),
                    "AAACGGTA".to_owned()
                ]
            ),
            2
        );
        assert_eq!(
            Solution::min_mutation(
                "AAAAACCC".to_owned(),
                "AACCCCCC".to_owned(),
                vec![
                    "AAAACCCC".to_owned(),
                    "AAACCCCC".to_owned(),
                    "AACCCCCC".to_owned()
                ]
            ),
            3
        );
    }
}
