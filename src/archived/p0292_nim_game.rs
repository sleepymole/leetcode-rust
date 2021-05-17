#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_win_nim() {
        assert!(!Solution::can_win_nim(4));
        assert!(Solution::can_win_nim(1));
        assert!(Solution::can_win_nim(2));
    }
}
