#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g = g;
        g.sort_unstable();
        let mut s = s;
        s.sort_unstable();
        let mut n = 0;
        let (mut i, mut j) = (0, 0);
        while i < g.len() && j < s.len() {
            if g[i] <= s[j] {
                i += 1;
                j += 1;
                n += 1;
            } else {
                j += 1;
            }
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_content_children() {
        assert_eq!(
            Solution::find_content_children(vec![1, 2, 3], vec![1, 1]),
            1
        );
        assert_eq!(
            Solution::find_content_children(vec![1, 2], vec![1, 2, 3]),
            2
        );
    }
}
