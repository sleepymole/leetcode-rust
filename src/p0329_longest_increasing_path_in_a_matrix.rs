#![allow(dead_code)]
pub struct Solution;

impl Solution {
    fn dfs(x: usize, y: usize, f: &mut Vec<Vec<i32>>, matrix: &Vec<Vec<i32>>) -> i32 {
        if f[x][y] > 0 {
            return f[x][y];
        }
        let m = f.len();
        let n = f[0].len();
        let mut ans = 0;
        if x > 0 && matrix[x - 1][y] > matrix[x][y] {
            if f[x - 1][y] == 0 {
                Solution::dfs(x - 1, y, f, matrix);
            }
            ans = ans.max(f[x - 1][y]);
        }
        if x + 1 < m && matrix[x + 1][y] > matrix[x][y] {
            if f[x + 1][y] == 0 {
                Solution::dfs(x + 1, y, f, matrix);
            }
            ans = ans.max(f[x + 1][y]);
        }
        if y > 0 && matrix[x][y - 1] > matrix[x][y] {
            if f[x][y - 1] == 0 {
                Solution::dfs(x, y - 1, f, matrix);
            }
            ans = ans.max(f[x][y - 1]);
        }
        if y + 1 < n && matrix[x][y + 1] > matrix[x][y] {
            if f[x][y + 1] == 0 {
                Solution::dfs(x, y + 1, f, matrix);
            }
            ans = ans.max(f[x][y + 1]);
        }
        f[x][y] = ans + 1;
        f[x][y]
    }

    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut f = vec![vec![0; n]; m];
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                ans = ans.max(Solution::dfs(i, j, &mut f, &matrix));
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_increasing_path() {
        assert_eq!(
            Solution::longest_increasing_path(vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]]),
            4
        );
        assert_eq!(
            Solution::longest_increasing_path(vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]]),
            4
        );
        assert_eq!(Solution::longest_increasing_path(vec![vec![1]]), 1);
    }
}
