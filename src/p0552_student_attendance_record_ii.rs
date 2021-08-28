#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn check_record(n: i32) -> i32 {
        if n == 1 {
            return 3;
        }
        const P: i64 = 1000000007i64;
        let n = n as usize;
        let mut f = vec![0; n + 1];
        f[0] = 1;
        f[1] = 1;
        f[2] = 2;
        for i in 3..=n {
            f[i] = (f[i - 1] + f[i - 2] + f[i - 3]) % P;
        }
        let sum_last_three = |n: usize| {
            if n == 0 {
                1
            } else if n == 1 {
                2
            } else {
                (f[n] + f[n - 1] + f[n - 2]) % P
            }
        };
        let mut ways = sum_last_three(n);
        for i in 1..=n {
            let a = sum_last_three(i - 1);
            let b = sum_last_three(n - i);
            ways = (ways + a * b) % P;
        }
        ways as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_record() {
        assert_eq!(Solution::check_record(2), 8);
        assert_eq!(Solution::check_record(1), 3);
        assert_eq!(Solution::check_record(10101), 183236316);
    }
}
