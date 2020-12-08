#![allow(dead_code)]
pub struct Solution;

use std::char;

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        if n == 1 {
            return "1".to_owned();
        }
        let mut x = 1;
        for i in 1..=n - 1 {
            x *= i;
        }
        let lead = (k - 1) / x + 1;
        let remain = Solution::get_permutation(n - 1, k - (k - 1) / x * x);
        let mut ans = String::new();
        ans.push(char::from_digit(lead as u32, 10).unwrap());
        for c in remain.chars() {
            let d = c as i32 - '0' as i32;
            if d >= lead {
                ans.push(char::from_digit((d + 1) as u32, 10).unwrap());
            } else {
                ans.push(c);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_permutation() {
        // assert_eq!(Solution::get_permutation(3, 3), "213".to_owned());
        assert_eq!(Solution::get_permutation(4, 9), "2314".to_owned());
        assert_eq!(Solution::get_permutation(3, 1), "123".to_owned());
    }
}
