#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        const INT32_MAX: i32 = 0x7fffffff;
        let mut x = x;
        let neg = if x >= 0 {
            false
        } else {
            if x == 1 << 31 {
                return 0;
            }
            x = -x;
            true
        };
        let mut r = 0;
        while x > 0 {
            if r > (INT32_MAX - x % 10) / 10 {
                return 0;
            }
            r = r * 10 + x % 10;
            x /= 10;
        }
        if neg {
            r = -r;
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
    }
}
