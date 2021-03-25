#![allow(dead_code)]
pub struct Solution;

use std::collections::BTreeSet;
use std::i32;

impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let mut primes = primes;
        primes.sort_unstable();
        let n = n as usize;
        let mut q = BTreeSet::new();
        let mut ans = 0;
        q.insert(1);
        for i in 0..n {
            if let Some(&x) = q.range(0..).next() {
                q.remove(&x);
                for &p in &primes {
                    if x > i32::MAX / p
                        || q.len() > n - i - 1 && p * x >= *q.range(0..).rev().next().unwrap()
                    {
                        break;
                    }
                    q.insert(p * x);
                }
                while q.len() > n - i - 1 {
                    if let Some(&x) = q.range(0..).rev().next() {
                        q.remove(&x);
                    }
                }
                ans = x;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_super_ugly_number() {
        assert_eq!(Solution::nth_super_ugly_number(12, vec![2, 7, 13, 19]), 32);
        assert_eq!(Solution::nth_super_ugly_number(1, vec![2, 3, 5]), 1);
    }
}
