#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let n = s.len();
        let mut f = vec![vec![0; n]; n];
        f[0][0] = 1;
        for i in 1..n {
            f[i][i] = 1;
            for j in (0..i).rev() {
                if s[i] == s[j] {
                    f[j][i] = if i - j > 1 { f[j + 1][i - 1] } else { 0 } + 2;
                } else {
                    f[j][i] = f[j + 1][i].max(f[j][i - 1]);
                }
            }
        }
        f[0][s.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome_subseq() {
        assert_eq!(Solution::longest_palindrome_subseq("bbbab".to_owned()), 4);
        assert_eq!(Solution::longest_palindrome_subseq("cbbd".to_owned()), 2);
    }
}
