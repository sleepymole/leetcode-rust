#![allow(dead_code)]
pub struct Solution {}

use std::fmt;
use std::ops::Add;

#[derive(Debug)]
struct Fraction {
    a: i32,
    b: i32,
}

impl Fraction {
    fn new(a: i32, b: i32) -> Self {
        let d = Fraction::gcd(a, b);
        let (mut a, mut b) = (a / d, b / d);
        if b < 0 {
            a = -a;
            b = -b;
        }
        Fraction { a, b }
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let t = a;
            a = b;
            b = t % b;
        }
        a
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Fraction) -> Self::Output {
        Fraction::new(self.a * rhs.b + self.b * rhs.a, self.b * rhs.b)
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.a, self.b)
    }
}

impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        let mut res = Fraction::new(0, 1);
        let s: Vec<char> = expression.chars().collect();
        let mut i = 0;
        while i < s.len() {
            let mut sign = 1;
            while i < s.len() && s[i] == '-' || s[i] == '+' {
                if s[i] == '-' {
                    sign = -sign;
                }
                i += 1;
            }
            let (mut a, mut b) = (0, 0);
            while i < s.len() && s[i].is_ascii_digit() {
                a = a * 10 + s[i] as i32 - '0' as i32;
                i += 1;
            }
            i += 1;
            while i < s.len() && s[i].is_ascii_digit() {
                b = b * 10 + s[i] as i32 - '0' as i32;
                i += 1;
            }
            res = res + Fraction::new(sign * a, b);
        }
        res.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fraction_addition() {
        assert_eq!(
            Solution::fraction_addition("-1/2+1/2".to_owned()),
            "0/1".to_owned()
        );
        assert_eq!(
            Solution::fraction_addition("-1/2+1/2+1/3".to_owned()),
            "1/3".to_owned()
        );
        assert_eq!(
            Solution::fraction_addition("1/3-1/2".to_owned()),
            "-1/6".to_owned()
        );
        assert_eq!(
            Solution::fraction_addition("5/3+1/3".to_owned()),
            "2/1".to_owned()
        );
    }
}
