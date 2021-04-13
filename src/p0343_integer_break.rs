#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n == 2 {
            return 1;
        }
        if n == 3 {
            return 2;
        }
        let pow = |mut x: i32, mut n: i32| {
            let mut r = 1;
            while n > 0 {
                if (n & 1) > 0 {
                    r *= x;
                }
                x = x * x;
                n >>= 1;
            }
            r
        };
        match n % 3 {
            0 => pow(3, n / 3),
            1 => 4 * pow(3, (n - 4) / 3),
            2 => 2 * pow(3, (n - 2) / 3),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_break() {
        assert_eq!(Solution::integer_break(2), 1);
        assert_eq!(Solution::integer_break(10), 36);
    }
}
