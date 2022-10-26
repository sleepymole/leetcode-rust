#![allow(dead_code)]
pub struct Solution;

use std::*;

impl Solution {
    fn abs(x: i32) -> u32 {
        if x >= 0 {
            return x as u32;
        }
        if x == i32::MIN {
            return 1 << 31;
        }
        -x as u32
    }

    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let neg = dividend ^ divisor < 0;
        let mut a = Solution::abs(dividend);
        let b = Solution::abs(divisor);
        let mut ans: u32 = 0;
        while a >= b {
            let (mut sum, mut cnt) = (b, 1);
            while sum <= a - sum {
                sum += sum;
                cnt += cnt;
            }
            ans += cnt;
            a -= sum;
        }
        if neg {
            return -(ans as i32);
        } else if ans == 1 << 31 {
            return i32::MAX;
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_divide() {
        assert_eq!(Solution::divide(10, 3), 3);
        assert_eq!(Solution::divide(7, -3), -2);
        assert_eq!(Solution::divide(0, 1), 0);
        assert_eq!(Solution::divide(i32::MAX, 1), i32::MAX);
        assert_eq!(Solution::divide(i32::MIN, -1), i32::MAX);
    }
}
