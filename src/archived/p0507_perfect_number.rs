#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        if num == 1 {
            return false;
        }
        let mut sum = 1;
        let mut i = 2;
        while i <= num / i {
            if num % i == 0 {
                sum += i;
                if num / i != i {
                    sum += num / i;
                }
            }
            i += 1;
        }
        sum == num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_perfect_number() {
        assert!(Solution::check_perfect_number(28));
        assert!(Solution::check_perfect_number(6));
        assert!(Solution::check_perfect_number(496));
        assert!(Solution::check_perfect_number(8128));
        assert!(!Solution::check_perfect_number(2));
    }
}
