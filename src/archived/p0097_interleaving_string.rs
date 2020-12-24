#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let s1 = s1.into_bytes();
        let s2 = s2.into_bytes();
        let s3 = s3.into_bytes();
        let mut f = Vec::new();
        f.resize(s1.len() + 1, vec![]);
        for x in f.iter_mut() {
            x.resize(s2.len() + 1, false);
        }
        f[0][0] = true;
        for i in 1..=s1.len() {
            if s1[i - 1] == s3[i - 1] {
                f[i][0] = true;
            } else {
                break;
            }
        }
        for j in 1..=s2.len() {
            if s2[j - 1] == s3[j - 1] {
                f[0][j] = true;
            } else {
                break;
            }
        }
        for i in 1..=s1.len() {
            for j in 1..=s2.len() {
                if f[i - 1][j] && s1[i - 1] == s3[i + j - 1]
                    || f[i][j - 1] && s2[j - 1] == s3[i + j - 1]
                {
                    f[i][j] = true;
                }
            }
        }
        f[s1.len()][s2.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_interleave() {
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_owned(),
                "dbbca".to_owned(),
                "aadbbcbcac".to_owned()
            ),
            true
        );
        assert_eq!(
            Solution::is_interleave(
                "aabcc".to_owned(),
                "dbbca".to_owned(),
                "aadbbbaccc".to_owned()
            ),
            false
        );
        assert_eq!(
            Solution::is_interleave("".to_owned(), "".to_owned(), "".to_owned()),
            true
        );
        assert_eq!(
            Solution::is_interleave("db".to_owned(), "b".to_owned(), "cbb".to_owned()),
            false
        );
    }
}
