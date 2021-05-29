#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let (mut l, mut r) = (0, n as i64 + 1);
        while l < r {
            let m = (l + r) / 2;
            if m * (m + 1) / 2 > n as i64 {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l as i32 - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arrange_coins() {
        assert_eq!(Solution::arrange_coins(5), 2);
        assert_eq!(Solution::arrange_coins(8), 3);
        assert_eq!(Solution::arrange_coins(1), 1);
    }
}
