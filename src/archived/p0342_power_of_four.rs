#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        n > 0 && (n & (n - 1)) == 0 && (n & 0x55555555) > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_four() {
        assert_eq!(Solution::is_power_of_four(16), true);
        assert_eq!(Solution::is_power_of_four(5), false);
        assert_eq!(Solution::is_power_of_four(1), true);
    }
}
