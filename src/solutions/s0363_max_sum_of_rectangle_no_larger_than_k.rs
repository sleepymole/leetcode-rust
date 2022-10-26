#![allow(dead_code)]
pub struct Solution;

use std::collections::BTreeSet;
use std::i32;

impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut max_sum = i32::MIN;
        for l in 0..m {
            let mut sum = vec![0; n];
            for r in l..m {
                let mut s = BTreeSet::new();
                s.insert(0);
                for i in (1..n).rev() {
                    sum[i] -= sum[i - 1];
                }
                for i in 0..n {
                    sum[i] += matrix[r][i];
                    if i > 0 {
                        sum[i] += sum[i - 1];
                    }
                    if let Some(&prev) = s.range(sum[i] - k..).next() {
                        max_sum = max_sum.max(sum[i] - prev);
                    }
                    s.insert(sum[i]);
                }
            }
        }
        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sum_submatrix() {
        assert_eq!(
            Solution::max_sum_submatrix(vec![vec![1, 0, 1], vec![0, -2, 3]], 2),
            2
        );
        assert_eq!(Solution::max_sum_submatrix(vec![vec![2, 2, -1]], 3), 3);
        assert_eq!(Solution::max_sum_submatrix(vec![vec![2, 2, -1]], 0), -1);
    }
}
