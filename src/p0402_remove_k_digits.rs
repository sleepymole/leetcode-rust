#![allow(dead_code)]
pub struct Solution;

use std::char;

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut k = k;
        let mut stack = Vec::new();
        for d in num.chars().map(|c| c as i32 - '0' as i32) {
            while k > 0 && !stack.is_empty() && d < *stack.last().unwrap() {
                stack.pop();
                k -= 1;
            }
            if !stack.is_empty() || d != 0 {
                stack.push(d);
            }
        }
        while k > 0 && !stack.is_empty() {
            stack.pop();
            k -= 1;
        }
        if !stack.is_empty() {
            stack
                .iter()
                .map(|&d| char::from_u32(d as u32 + '0' as u32).unwrap())
                .collect()
        } else {
            "0".to_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_kdigits() {
        assert_eq!(
            Solution::remove_kdigits("1432219".to_owned(), 3),
            "1219".to_owned()
        );
        assert_eq!(
            Solution::remove_kdigits("10200".to_owned(), 1),
            "200".to_owned()
        );
        assert_eq!(Solution::remove_kdigits("10".to_owned(), 2), "0".to_owned());
        assert_eq!(Solution::remove_kdigits("9".to_owned(), 1), "0".to_owned());
    }
}
