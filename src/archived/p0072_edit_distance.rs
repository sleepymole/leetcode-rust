#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.into_bytes();
        let word2 = word2.into_bytes();
        let m = word1.len();
        let n = word2.len();
        let mut f: Vec<Vec<i32>> = Vec::new();
        f.resize(m + 1, vec![]);
        for l in f.iter_mut() {
            l.resize(n + 1, 0);
        }
        for i in 1..=m {
            f[i][0] = i as i32;
        }
        for j in 1..=n {
            f[0][j] = j as i32;
        }
        for i in 1..=m {
            for j in 1..=n {
                f[i][j] = f[i - 1][j].min(f[i][j - 1]).min(f[i - 1][j - 1]) + 1;
                if word1[i - 1] == word2[j - 1] {
                    f[i][j] = f[i][j].min(f[i - 1][j - 1]);
                }
            }
        }
        f[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_distance() {
        assert_eq!(
            Solution::min_distance("horse".to_owned(), "ros".to_owned()),
            3
        );
        assert_eq!(
            Solution::min_distance("intention".to_owned(), "execution".to_owned()),
            5
        );
    }
}
