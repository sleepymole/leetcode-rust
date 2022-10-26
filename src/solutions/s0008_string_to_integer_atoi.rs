#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        const INT_MAX: i32 = 0x7fffffff;
        const INT_MIN: i32 = 1 << 31;
        let mut start = false;
        let mut flag: i32 = 1;
        let mut n: i32 = 0;
        for c in str.chars() {
            if !start {
                if c == ' ' {
                    continue;
                }
                start = true;
                if c == '-' {
                    flag = -1;
                    continue;
                }
                if c == '+' {
                    flag = 1;
                    continue;
                }
            }
            if !('0'..='9').contains(&c) {
                return flag * n;
            }
            let d = c.to_digit(10).unwrap() as i32;

            if flag < 0 && d == 8 && n >= (INT_MAX - d + 1) / 10 {
                return INT_MIN;
            }
            if n > (INT_MAX - d) / 10 {
                if flag > 0 {
                    return INT_MAX;
                } else {
                    return INT_MIN;
                }
            }
            n = n * 10 + d;
        }
        flag * n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_atoi() {
        assert_eq!(Solution::my_atoi("42".to_owned()), 42);
        assert_eq!(Solution::my_atoi("   -42".to_owned()), -42);
        assert_eq!(Solution::my_atoi("4193 with words".to_owned()), 4193);
        assert_eq!(Solution::my_atoi("words and 987".to_owned()), 0);
        assert_eq!(Solution::my_atoi("-91283472332".to_owned()), -2147483648);
    }
}
