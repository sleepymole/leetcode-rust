#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let mut segs = 0;
        let mut inword = false;
        for c in s.chars() {
            if c == ' ' {
                inword = false;
            } else if !inword {
                segs += 1;
                inword = true;
            }
        }
        segs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_segments() {
        assert_eq!(
            Solution::count_segments("Hello, my name is John".to_owned()),
            5
        );
        assert_eq!(Solution::count_segments("Hello".to_owned()), 1);
    }
}
