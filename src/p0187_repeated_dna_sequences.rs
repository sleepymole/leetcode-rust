#![allow(dead_code)]
pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let s: Vec<char> = s.chars().collect();
        if s.len() < 10 {
            return vec![];
        }
        let mut ans = HashSet::new();
        let mut hs = HashSet::new();
        for i in 0..=s.len() - 10 {
            let mut key = 0;
            for j in i..i + 10 {
                key = key * 26 + s[j] as u64 - 'A' as u64;
            }
            if hs.contains(&key) {
                let mut ss = String::new();
                for j in i..i + 10 {
                    ss.push(s[j]);
                }
                ans.insert(ss);
            } else {
                hs.insert(key);
            }
        }
        let mut ans: Vec<String> = ans.into_iter().collect();
        ans.sort_unstable();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_repeated_dna_sequences() {
        assert_eq!(
            Solution::find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_owned()),
            vec!["AAAAACCCCC".to_owned(), "CCCCCAAAAA".to_owned()]
        );
        assert_eq!(
            Solution::find_repeated_dna_sequences("AAAAAAAAAAAAA".to_owned()),
            vec!["AAAAAAAAAA".to_owned()]
        );
    }
}
