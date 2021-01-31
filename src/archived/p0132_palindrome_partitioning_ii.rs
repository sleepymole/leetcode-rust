#![allow(dead_code)]
pub struct Solution;

use std::i32;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        let mut vs = vec!['#'];
        for c in s.chars() {
            vs.push(c);
            vs.push('#');
        }
        let mut f = vec![0; vs.len()];
        let mut pr = (0, 0);
        for i in 1..vs.len() {
            if pr.0 + pr.1 <= i {
                let mut j = 0;
                while i > j && i + j + 1 < vs.len() && vs[i - j - 1] == vs[i + j + 1] {
                    j += 1;
                }
                pr = (i, j);
                f[i] = j;
            } else {
                let mut j = f[2 * pr.0 - i].min(pr.0 + pr.1 - i);
                while i > j && i + j + 1 < vs.len() && vs[i - j - 1] == vs[i + j + 1] {
                    j += 1;
                }
                f[i] = j;
                if pr.0 + pr.1 < i + j {
                    pr = (i, j);
                }
            }
        }
        let s: Vec<char> = s.chars().collect();
        let mut min_cuts = vec![i32::MAX; s.len()];
        for i in 0..s.len() {
            if f[i + 1] >= i {
                min_cuts[i] = 0;
                continue;
            }
            for j in 1..=i {
                if f[i + j + 1] >= i - j {
                    min_cuts[i] = min_cuts[i].min(min_cuts[j - 1] + 1);
                }
            }
        }
        min_cuts[min_cuts.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cut() {
        assert_eq!(Solution::min_cut("aab".to_owned()), 1);
        assert_eq!(Solution::min_cut("a".to_owned()), 0);
        assert_eq!(Solution::min_cut("ab".to_owned()), 1);
    }
}
