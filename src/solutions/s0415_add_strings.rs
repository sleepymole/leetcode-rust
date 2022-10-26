#![allow(dead_code)]
pub struct Solution;

use std::char;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let num1: Vec<u32> = num1.chars().rev().map(|c| c as u32 - '0' as u32).collect();
        let num2: Vec<u32> = num2.chars().rev().map(|c| c as u32 - '0' as u32).collect();
        let mut res = Vec::new();
        let mut carry = 0;
        for i in 0..num1.len().max(num2.len()) {
            if i < num1.len() {
                carry += num1[i];
            }
            if i < num2.len() {
                carry += num2[i];
            }
            res.push(carry % 10);
            carry /= 10;
        }
        if carry > 0 {
            res.push(carry);
        }
        res.into_iter()
            .rev()
            .map(|x| char::from_u32(x + '0' as u32).unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_string() {
        assert_eq!(
            Solution::add_strings("11".to_owned(), "123".to_owned()),
            "134".to_owned()
        );
        assert_eq!(
            Solution::add_strings("456".to_owned(), "77".to_owned()),
            "533".to_owned()
        );
        assert_eq!(
            Solution::add_strings("0".to_owned(), "0".to_owned()),
            "0".to_owned()
        );
    }
}
