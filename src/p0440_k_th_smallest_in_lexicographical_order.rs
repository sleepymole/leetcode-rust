#![allow(dead_code)]
pub struct Solution;

impl Solution {
    fn search(n: i32, k: i32, p: i32) -> (i32, i32) {
        if p > 0 {
            let mut count = 1;
            let mut mul = 1;
            while p * mul <= n / 10 {
                mul *= 10;
                count += mul;
            }
            if n / mul == p {
                count = count - mul + n - p * mul + 1;
            }
            if count < k + 1 {
                return (0, count);
            }
        }

        if k == 0 {
            return (p, 0);
        }
        let mut i = 0;
        if p == 0 {
            i = 1;
        }
        let mut total = 1;
        while i < 10 {
            if p > (n - i) / 10 {
                break;
            }
            let (target, count) = Solution::search(n, k - total, p * 10 + i);
            if target > 0 {
                return (target, 0);
            }
            total += count;
            i += 1;
        }
        (0, total)
    }

    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        Solution::search(n, k, 0).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_kth_number() {
        assert_eq!(Solution::find_kth_number(13, 2), 10);
        assert_eq!(Solution::find_kth_number(1, 1), 1);
        assert_eq!(Solution::find_kth_number(804289384, 42641503), 138377349);
        assert_eq!(Solution::find_kth_number(1692778, 1545580), 867519);
    }
}
