#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        if max_move <= 0 {
            return 0;
        }
        const P: i32 = 1000000007;
        let (m, n) = (m as usize, n as usize);
        let mut f = vec![vec![vec![0; max_move as usize]; n]; m];
        f[start_row as usize][start_column as usize][0] = 1;
        for k in 1..max_move as usize {
            for i in 0..m {
                for j in 0..n {
                    if i > 0 {
                        f[i][j][k] = (f[i][j][k] + f[i - 1][j][k - 1]) % P;
                    }
                    if i + 1 < m {
                        f[i][j][k] = (f[i][j][k] + f[i + 1][j][k - 1]) % P;
                    }
                    if j > 0 {
                        f[i][j][k] = (f[i][j][k] + f[i][j - 1][k - 1]) % P;
                    }
                    if j + 1 < n {
                        f[i][j][k] = (f[i][j][k] + f[i][j + 1][k - 1]) % P;
                    }
                }
            }
        }
        let mut paths = 0;
        for i in 0..m {
            for k in 0..max_move as usize {
                paths = (paths + f[i][0][k]) % P;
                paths = (paths + f[i][n - 1][k]) % P;
            }
        }
        for j in 0..n {
            for k in 0..max_move as usize {
                paths = (paths + f[0][j][k]) % P;
                paths = (paths + f[m - 1][j][k]) % P;
            }
        }
        paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_paths() {
        assert_eq!(Solution::find_paths(2, 2, 2, 0, 0), 6);
        assert_eq!(Solution::find_paths(1, 3, 3, 0, 1), 12);
    }
}
