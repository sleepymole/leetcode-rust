#![allow(dead_code)]
pub struct Solution {}

use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(PartialEq, Copy, Clone)]
struct Complex {
    a: i32,
    b: i32,
}

impl Complex {
    fn new(a: i32, b: i32) -> Self {
        Complex { a, b }
    }

    fn mul(self, rhs: Complex) -> Self {
        Complex {
            a: self.a * rhs.a - self.b * rhs.b,
            b: self.a * rhs.b + self.b * rhs.a,
        }
    }
}

impl FromStr for Complex {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.splitn(2, '+').collect();
        let a = v[0].parse::<i32>()?;
        let b = v[1].strip_suffix('i').unwrap().parse::<i32>()?;
        Ok(Complex { a, b })
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}+{}i", self.a, self.b)
    }
}

impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let c1 = num1.parse::<Complex>().unwrap();
        let c2 = num2.parse::<Complex>().unwrap();
        c1.mul(c2).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_number_multiply() {
        assert_eq!(
            Solution::complex_number_multiply("1+1i".to_owned(), "1+1i".to_owned()),
            "0+2i".to_owned()
        );
        assert_eq!(
            Solution::complex_number_multiply("1+-1i".to_owned(), "1+-1i".to_owned()),
            "0+-2i".to_owned()
        );
    }
}
