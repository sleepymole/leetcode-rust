#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid[0][0] == 1 {
            return 0;
        }
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut f: Vec<Vec<i32>> = Vec::new();
        f.resize(m, vec![]);
        for l in f.iter_mut() {
            l.resize(n, 0);
        }
        f[0][0] = 1;
        for j in 1..n {
            if obstacle_grid[0][j] == 0 {
                f[0][j] = f[0][j - 1];
            } else {
                f[0][j] = 0;
            }
        }
        for i in 1..m {
            if obstacle_grid[i][0] == 0 {
                f[i][0] = f[i - 1][0];
            } else {
                f[i][0] = 0;
            }
        }
        for i in 1..m {
            for j in 1..n {
                if obstacle_grid[i][j] == 0 {
                    f[i][j] = f[i - 1][j] + f[i][j - 1];
                } else {
                    f[i][j] = 0;
                }
            }
        }
        f[m - 1][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_paths_with_obstacles() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]),
            2
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]),
            1
        )
    }
}
