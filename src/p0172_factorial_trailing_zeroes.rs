#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut ans = 0;
        let mut x = 5;
        while x <= n {
            ans += n / x;
            x *= 5;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trailing_zeroes() {
        assert_eq!(Solution::trailing_zeroes(3), 0);
        assert_eq!(Solution::trailing_zeroes(5), 1);
        assert_eq!(Solution::trailing_zeroes(0), 0);
    }
}
