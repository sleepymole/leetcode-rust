#![allow(dead_code)]
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    fn search(
        s1: &Vec<u8>,
        s2: &Vec<u8>,
        i: usize,
        j: usize,
        k: usize,
        cache: &mut HashMap<(usize, usize, usize), bool>,
    ) -> bool {
        if k == 0 {
            return true;
        }
        if k == 1 {
            return s1[i] == s2[j];
        }
        if let Some(&x) = cache.get(&(i, j, k)) {
            return x;
        }
        for x in 1..k {
            if Solution::search(s1, s2, i, j, x, cache)
                && Solution::search(s1, s2, i + x, j + x, k - x, cache)
                || Solution::search(s1, s2, i, j + k - x, x, cache)
                    && Solution::search(s1, s2, i + x, j, k - x, cache)
            {
                cache.insert((i, j, k), true);
                return true;
            }
        }
        cache.insert((i, j, k), false);
        false
    }

    pub fn is_scramble(s1: String, s2: String) -> bool {
        let s1 = s1.into_bytes();
        let s2 = s2.into_bytes();
        if s1.len() != s2.len() {
            return false;
        }
        let mut cache = HashMap::new();
        Solution::search(&s1, &s2, 0, 0, s1.len(), &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_scramble() {
        assert!(Solution::is_scramble(
            "great".to_owned(),
            "rgeat".to_owned()
        ));
        assert!(!Solution::is_scramble(
            "abcde".to_owned(),
            "caebd".to_owned()
        ));
        assert!(Solution::is_scramble("a".to_owned(), "a".to_owned()));
    }
}
