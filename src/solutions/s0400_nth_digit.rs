#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut n = n;
        let (mut d, mut ds, mut dc) = (1, 1, 9);
        while dc < n / d && d * dc != n {
            n -= d * dc;
            d += 1;
            ds *= 10;
            dc *= 10;
        }
        let mut x = ds + (n - 1) / d;
        for _ in 1..d - (n - 1) % d {
            x /= 10;
        }
        x % 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_nth_digit() {
        assert_eq!(Solution::find_nth_digit(3), 3);
        assert_eq!(Solution::find_nth_digit(11), 0);
    }
}
