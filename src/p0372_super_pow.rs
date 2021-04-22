#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        let mut a = a % 1337;
        let mut b = b;
        let mut res = 1;
        while let Some(x) = b.pop() {
            for _ in 0..x {
                res = (res * a) % 1337;
            }
            let mut t = 1;
            for _ in 0..10 {
                t = (t * a) % 1337;
            }
            a = t;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_super_pow() {
        assert_eq!(Solution::super_pow(2, vec![3]), 8);
        assert_eq!(Solution::super_pow(2, vec![1, 0]), 1024);
        assert_eq!(Solution::super_pow(1, vec![4, 3, 3, 8, 5, 2]), 1);
        assert_eq!(Solution::super_pow(2147483647, vec![2, 0, 0]), 1198);
    }
}
