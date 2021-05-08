#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        n - Solution::last_remaining(n / 2) * 2 + 2 - n % 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_remaining() {
        assert_eq!(Solution::last_remaining(9), 6);
        assert_eq!(Solution::last_remaining(1), 1);
    }
}
