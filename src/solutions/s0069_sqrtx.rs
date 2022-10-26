#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x <= 1 {
            return x;
        }
        let (mut l, mut r) = (0, x);
        while l < r {
            let m = (l + r) / 2;
            if m <= x / m {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_sqrt() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
        assert_eq!(Solution::my_sqrt(0), 0);
        assert_eq!(Solution::my_sqrt(1), 1);
        assert_eq!(Solution::my_sqrt(2), 1);
        assert_eq!(Solution::my_sqrt(i32::MAX), 46340);
    }
}
