#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let (mut a, mut b) = (a, b);
        while b != 0 {
            let c = a & b;
            a ^= b;
            b = c << 1;
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_num() {
        assert_eq!(Solution::get_sum(1, 2), 3);
        assert_eq!(Solution::get_sum(2, 3), 5);
    }
}
