#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    fn count_bits(mut x: i32) -> i32 {
        let mut bits = 0;
        while x > 0 {
            bits += 1;
            x >>= 1;
        }
        bits
    }
    fn count_fixed_len(len: i32) -> i32 {
        let mut a = 1;
        let mut b = 0;
        for _ in 0..len {
            let t = a;
            a += b;
            b = t;
        }
        a + b
    }

    pub fn find_integers(n: i32) -> i32 {
        if n <= 1 {
            return n + 1;
        }
        let bits = Solution::count_bits(n);
        let count = Solution::count_fixed_len(bits - 1);
        if (n & (1 << (bits - 2))) > 0 {
            count + Solution::count_fixed_len(bits - 2)
        } else {
            count + Solution::find_integers(n ^ (1 << (bits - 1)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_integers() {
        assert_eq!(Solution::find_integers(5), 5);
        assert_eq!(Solution::find_integers(1), 2);
        assert_eq!(Solution::find_integers(2), 3);
    }
}
