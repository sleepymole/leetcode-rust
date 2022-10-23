#![allow(dead_code)]
pub struct Solution;

use std::char;
use std::collections::HashMap;
use std::i32;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let mut p = numerator as i64;
        let q = denominator as i64;
        if p % q == 0 {
            return (p / q).to_string();
        }
        let mut ans = Vec::new();
        if p.signum() * q.signum() < 0 {
            ans.push('-');
        }
        let mut s = (p / q).abs().to_string().chars().collect();
        ans.append(&mut s);
        p %= q;
        ans.push('.');
        let dot_pos = ans.len() - 1;
        let mut m = HashMap::new();
        let mut pos = dot_pos;
        while p != 0 && !m.contains_key(&p) {
            m.insert(p, pos);
            p *= 10;
            ans.push(char::from_digit((p / q).unsigned_abs() as u32, 10).unwrap());
            p %= q;
            pos += 1;
        }
        if p == 0 {
            return ans.iter().collect();
        }
        ans.insert(*m.get(&p).unwrap() + 1, '(');
        ans.push(')');
        ans.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fraction_to_decimal() {
        assert_eq!(Solution::fraction_to_decimal(1, 2), "0.5".to_owned());
        assert_eq!(Solution::fraction_to_decimal(2, 1), "2".to_owned());
        assert_eq!(Solution::fraction_to_decimal(2, 3), "0.(6)".to_owned());
        assert_eq!(Solution::fraction_to_decimal(4, 333), "0.(012)".to_owned());
        assert_eq!(Solution::fraction_to_decimal(1, 5), "0.2".to_owned());
        assert_eq!(Solution::fraction_to_decimal(-50, 8), "-6.25".to_owned());
        assert_eq!(Solution::fraction_to_decimal(7, -12), "-0.58(3)".to_owned());
        assert_eq!(
            Solution::fraction_to_decimal(-1, -2147483648),
            "0.0000000004656612873077392578125".to_owned()
        );
    }
}
