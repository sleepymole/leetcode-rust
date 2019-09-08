#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        if x == 0 {
            return true;
        }
        let mut x = x;
        let mut digits: Vec<i32> = Vec::new();
        while x > 0 {
            digits.push(x % 10);
            x /= 10;
        }
        let (mut i, mut j) = (0, digits.len() - 1);
        while i < j {
            if digits[i] != digits[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(10), false);
    }
}
