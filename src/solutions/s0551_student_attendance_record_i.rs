#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn check_record(s: String) -> bool {
        if s.chars().filter(|&c| c == 'A').count() > 1 {
            return false;
        }
        let mut lates = 0;
        for c in s.chars() {
            if c == 'L' {
                lates += 1;
                if lates >= 3 {
                    return false;
                }
            } else {
                lates = 0;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_record() {
        assert!(Solution::check_record("PPALLP".to_owned()));
        assert!(!Solution::check_record("PPALLL".to_owned()));
    }
}
