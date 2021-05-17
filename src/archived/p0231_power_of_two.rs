#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        n & (n - 1) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_two() {
        assert!(Solution::is_power_of_two(1));
        assert!(Solution::is_power_of_two(16));
        assert!(!Solution::is_power_of_two(3));
        assert!(Solution::is_power_of_two(4));
        assert!(!Solution::is_power_of_two(5));
    }
}
