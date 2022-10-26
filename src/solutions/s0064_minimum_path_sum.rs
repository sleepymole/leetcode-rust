#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut f: Vec<Vec<i32>> = Vec::new();
        f.resize(m, vec![]);
        for l in f.iter_mut() {
            l.resize(n, 0);
        }
        f[0][0] = grid[0][0];
        for i in 1..m {
            f[i][0] = grid[i][0] + f[i - 1][0];
        }
        for j in 1..n {
            f[0][j] = grid[0][j] + f[0][j - 1];
        }
        for i in 1..m {
            for j in 1..n {
                f[i][j] = f[i - 1][j].min(f[i][j - 1]) + grid[i][j];
            }
        }
        f[m - 1][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_path_sum() {
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            12
        );
    }
}
