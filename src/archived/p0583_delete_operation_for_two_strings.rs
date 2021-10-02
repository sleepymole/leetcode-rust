#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let s1: Vec<char> = word1.chars().collect();
        let s2: Vec<char> = word2.chars().collect();
        let mut f = vec![vec![0; s2.len() + 1]; s1.len() + 1];
        for i in 1..=s1.len() {
            for j in 1..=s2.len() {
                if s1[i - 1] == s2[j - 1] {
                    f[i][j] = f[i - 1][j - 1] + 1;
                } else {
                    f[i][j] = f[i][j - 1].max(f[i - 1][j]);
                }
            }
        }
        (s1.len() + s2.len() - f[s1.len()][s2.len()] * 2) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_distance() {
        assert_eq!(
            Solution::min_distance("sea".to_owned(), "eat".to_owned()),
            2
        );
        assert_eq!(
            Solution::min_distance("leetcode".to_owned(), "etco".to_owned()),
            4
        );
    }
}
