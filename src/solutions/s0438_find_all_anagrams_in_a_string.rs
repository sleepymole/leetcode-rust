#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if s.len() < p.len() {
            return Vec::new();
        }
        let mut mismatch = 0;
        let mut m = vec![0; 26];
        for c in p.chars().map(|c| c as usize - 'a' as usize) {
            m[c] += 1;
            if m[c] == 1 {
                mismatch += 1;
            }
        }
        let s: Vec<usize> = s.chars().map(|c| c as usize - 'a' as usize).collect();
        let mut res = Vec::new();
        let (mut i, mut j) = (0, 0);
        while j < s.len() {
            while j - i < p.len() {
                if m[s[j]] == 0 {
                    mismatch += 1;
                }
                m[s[j]] -= 1;
                if m[s[j]] == 0 {
                    mismatch -= 1;
                }
                j += 1;
            }
            if mismatch == 0 {
                res.push(i as i32);
            }
            if m[s[i]] == 0 {
                mismatch += 1;
            }
            m[s[i]] += 1;
            if m[s[i]] == 0 {
                mismatch -= 1;
            }
            i += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_anagrams() {
        assert_eq!(
            Solution::find_anagrams("cbaebabacd".to_owned(), "abc".to_owned()),
            vec![0, 6]
        );
        assert_eq!(
            Solution::find_anagrams("abab".to_owned(), "ab".to_owned()),
            vec![0, 1, 2]
        );
    }
}
