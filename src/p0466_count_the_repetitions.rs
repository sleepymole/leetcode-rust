#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let l1 = s1.len() * n1 as usize;
        let mut i = 0;
        let mut ends = vec![None; s1.len()];
        let mut m: usize = 0;
        while i < l1 {
            let mut j = 0;
            while i < l1 && j < s2.len() {
                if s1[i % s1.len()] == s2[j] {
                    i += 1;
                    j += 1;
                } else {
                    i += 1;
                }
            }
            if j < s2.len() {
                break;
            }
            m += 1;
            if let Some((ii, mm)) = ends[i % s1.len()] {
                let skip = (l1 - i) / (i - ii);
                i += skip * (i - ii);
                m += skip * (m - mm);
            } else {
                ends[i % s1.len()] = Some((i, m));
            }
        }
        m as i32 / n2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_max_repetitions() {
        assert_eq!(
            Solution::get_max_repetitions("acb".to_owned(), 4, "ab".to_owned(), 2),
            2
        );
        assert_eq!(
            Solution::get_max_repetitions("acb".to_owned(), 1, "acb".to_owned(), 1),
            1
        );
    }
}
