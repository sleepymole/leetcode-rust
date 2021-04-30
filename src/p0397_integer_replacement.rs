#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut ops = 0;
        let mut n = n as usize;
        while n != 1 {
            if n % 2 == 0 {
                n >>= 1;
                ops += 1;
            } else if n % 4 == 1 {
                n -= 1;
                ops += 1;
            } else if n == 3 {
                ops += 2;
                n >>= 1;
            } else if n % 4 == 3 {
                n += 1;
                ops += 1;
            }
        }
        ops
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_replacement() {
        assert_eq!(Solution::integer_replacement(8), 3);
        assert_eq!(Solution::integer_replacement(7), 4);
        assert_eq!(Solution::integer_replacement(4), 2);
    }
}
