#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn is_ugly(num: i32) -> bool {
        if num <= 0 {
            return false;
        }
        let mut num = num;
        while num > 1 {
            if num % 2 == 0 {
                num /= 2;
            } else if num % 3 == 0 {
                num /= 3;
            } else if num % 5 == 0 {
                num /= 5;
            } else {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ugly() {
        assert_eq!(Solution::is_ugly(6), true);
        assert_eq!(Solution::is_ugly(8), true);
        assert_eq!(Solution::is_ugly(14), false);
    }
}
