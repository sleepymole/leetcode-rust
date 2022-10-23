#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        if n < 10 {
            return i32::from(n >= 1);
        }
        let mut q = n / 10;
        let r = n % 10;
        let mut ans = Solution::count_digit_one(q - 1) * 10 + q;
        if r >= 1 {
            ans += 1;
        }
        let mut cnt = 0;
        while q > 0 {
            if q % 10 == 1 {
                cnt += 1;
            }
            q /= 10;
        }
        ans += cnt * (r + 1);
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_digit_one() {
        assert_eq!(Solution::count_digit_one(13), 6);
        assert_eq!(Solution::count_digit_one(0), 0);
    }
}
