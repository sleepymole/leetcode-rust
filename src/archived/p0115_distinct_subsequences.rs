#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.into_bytes();
        let t = t.into_bytes();
        let mut f = vec![vec![0; t.len() + 1]; s.len() + 1];
        for i in 0..=s.len() {
            f[i][0] = 1;
        }
        for i in 1..=s.len() {
            for j in 1..=t.len() {
                f[i][j] = f[i - 1][j];
                if s[i - 1] == t[j - 1] {
                    f[i][j] += f[i - 1][j - 1];
                }
            }
        }
        f[s.len()][t.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_distinct() {
        assert_eq!(
            Solution::num_distinct("rabbbit".to_owned(), "rabbit".to_owned()),
            3
        );
        assert_eq!(
            Solution::num_distinct("babgbag".to_owned(), "bag".to_owned()),
            5
        );
    }
}
