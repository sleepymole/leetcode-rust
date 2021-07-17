#![allow(dead_code)]
pub struct Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let mut left: BTreeMap<i32, i32> = BTreeMap::new();
        let mut right: BTreeMap<i32, i32> = BTreeMap::new();
        let add = |m: &mut BTreeMap<i32, i32>, v: i32| {
            *m.entry(v).or_insert(0) += 1;
        };
        let remove = |m: &mut BTreeMap<i32, i32>, v: i32| {
            if let Some(c) = m.get_mut(&v) {
                *c -= 1;
                if *c == 0 {
                    m.remove(&v);
                }
                true
            } else {
                false
            }
        };
        let mut llen = 0;
        let mut rlen = 0;
        let mut res = Vec::new();
        for (i, &x) in nums.iter().enumerate() {
            if let Some((&m, _)) = right.range(i32::MIN..).next() {
                if x <= m {
                    add(&mut left, x);
                    llen += 1;
                } else {
                    add(&mut right, x);
                    rlen += 1;
                }
            } else {
                add(&mut left, x);
                llen += 1;
            }
            if i >= k as usize {
                let v = nums[i - k as usize];
                if remove(&mut left, v) {
                    llen -= 1;
                } else if remove(&mut right, v) {
                    rlen -= 1;
                }
            }
            while llen > rlen + 1 {
                let max = *left.range(..=i32::MAX).rev().next().unwrap().0;
                remove(&mut left, max);
                add(&mut right, max);
                llen -= 1;
                rlen += 1;
            }
            while llen < rlen {
                let min = *right.range(i32::MIN..).next().unwrap().0;
                remove(&mut right, min);
                add(&mut left, min);
                llen += 1;
                rlen -= 1;
            }
            if i + 1 >= k as usize {
                let lmax = *left.range(..=i32::MAX).rev().next().unwrap().0;
                if llen > rlen {
                    res.push(lmax as f64);
                } else {
                    let rmin = *right.range(i32::MIN..).next().unwrap().0;
                    res.push((lmax as f64 + rmin as f64) / 2.0);
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_sliding_window() {
        assert_eq!(
            Solution::median_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![1.00000, -1.00000, -1.00000, 3.00000, 5.00000, 6.00000]
        );
        assert_eq!(
            Solution::median_sliding_window(vec![1, 2, 3, 4, 2, 3, 1, 4, 2], 3),
            vec![2.00000, 3.00000, 3.00000, 3.00000, 2.00000, 3.00000, 2.00000]
        );
        assert_eq!(
            Solution::median_sliding_window(vec![2147483647, 2147483647], 2),
            vec![2147483647.0]
        );
    }
}
