#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        let mut n = n;
        while n > 1 && n % 3 == 0 {
            n /= 3;
        }
        n == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_three() {
        assert!(Solution::is_power_of_three(27));
        assert!(!Solution::is_power_of_three(0));
        assert!(Solution::is_power_of_three(9));
        assert!(!Solution::is_power_of_three(45));
    }
}
