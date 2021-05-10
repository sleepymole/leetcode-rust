#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let (mut l, mut r) = (1, num);
        while l < r {
            let m = (l + r) / 2;
            if m >= num / m {
                r = m;
            } else {
                l = m + 1;
            }
        }
        num % l == 0 && num / l == l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_perfect_square() {
        assert_eq!(Solution::is_perfect_square(16), true);
        assert_eq!(Solution::is_perfect_square(14), false);
    }
}
