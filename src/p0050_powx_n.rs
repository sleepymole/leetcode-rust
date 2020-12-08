#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut x = x;
        let mut n = n;
        let mut ans = 1.0;
        let neg = n < 0;
        while n != 0 {
            if n % 2 != 0 {
                ans *= x;
            }
            x *= x;
            n /= 2;
        }
        if neg {
            ans = 1.0 / ans;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_pow() {
        assert_eq!((Solution::my_pow(2.0, 10) - 1024.0).abs() < 1e-10, true);
        assert_eq!((Solution::my_pow(2.1, 3) - 9.261).abs() < 1e-10, true);
        assert_eq!((Solution::my_pow(2.0, -2) - 0.25).abs() < 1e-10, true);
    }
}
