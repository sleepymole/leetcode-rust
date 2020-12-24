#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m - 1;
        let mut n = n - 1;
        let m = m + n;
        if n > m - n {
            n = m - n;
        }
        let mut ans = 1_u64;
        for i in 0..n {
            ans *= (m - i) as u64;
        }
        for i in 1..=n {
            ans /= i as u64;
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_paths() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
        assert_eq!(Solution::unique_paths(3, 2), 3);
        assert_eq!(Solution::unique_paths(7, 3), 28);
        assert_eq!(Solution::unique_paths(3, 3), 6);
    }
}
