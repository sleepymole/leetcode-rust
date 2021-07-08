#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn largest_palindrome(n: i32) -> i32 {
        if n == 1 {
            return 9;
        }
        let left_max = i32::pow(10, n as u32) - 1;
        for left in (0..=left_max).rev() {
            let p = Solution::gen_palindrome(left);
            for a in (1..=left_max as i64).rev() {
                let b = p / a;
                if a < b {
                    break;
                }
                if p % a == 0 {
                    return (p % 1337i64) as i32;
                }
            }
        }
        0
    }

    fn gen_palindrome(left: i32) -> i64 {
        let mut res = left as i64;
        let mut left = left as i64;
        while left > 0 {
            res = res * 10 + left % 10;
            left /= 10;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_palindrome() {
        assert_eq!(Solution::largest_palindrome(2), 987);
        assert_eq!(Solution::largest_palindrome(1), 9);
    }
}
