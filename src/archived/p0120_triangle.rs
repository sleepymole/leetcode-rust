#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut f = triangle.clone();
        for i in 1..triangle.len() {
            f[i][0] += f[i - 1][0];
            f[i][i] += f[i - 1][i - 1];
            for j in 1..i {
                f[i][j] += f[i - 1][j - 1].min(f[i - 1][j]);
            }
        }
        *f[f.len() - 1].iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_total() {
        assert_eq!(
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
            11
        );
        assert_eq!(Solution::minimum_total(vec![vec![-10]]), -10);
    }
}
