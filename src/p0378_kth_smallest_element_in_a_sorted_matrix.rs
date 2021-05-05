#![allow(dead_code)]
pub struct Solution;

impl Solution {
    fn search<F>(l: i32, r: i32, f: F) -> i32
    where
        F: Fn(i32) -> bool,
    {
        let (mut i, mut j) = (l, r);
        while i < j {
            let h = (i + j) / 2;
            if f(h) {
                j = h;
            } else {
                i = h + 1;
            }
        }
        i
    }

    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = matrix.len();
        let (l, r) = (matrix[0][0], matrix[n - 1][n - 1]);
        Solution::search(l, r, |target| {
            let mut count = 0;
            for rows in &matrix {
                count += Solution::search(0, n as i32, |i| rows[i as usize] > target);
            }
            count >= k
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_smallest() {
        assert_eq!(
            Solution::kth_smallest(vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]], 8),
            13
        );
        assert_eq!(Solution::kth_smallest(vec![vec![-5]], 1), -5);
    }
}
