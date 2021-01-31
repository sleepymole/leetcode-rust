#![allow(dead_code)]
pub struct Solution;

impl Solution {
    fn dfs(grid: &Vec<Vec<char>>, x: usize, y: usize, vis: &mut Vec<Vec<bool>>) {
        vis[x][y] = true;
        if x > 0 && grid[x - 1][y] == '1' && !vis[x - 1][y] {
            Solution::dfs(grid, x - 1, y, vis);
        }
        if x + 1 < grid.len() && grid[x + 1][y] == '1' && !vis[x + 1][y] {
            Solution::dfs(grid, x + 1, y, vis);
        }
        if y > 0 && grid[x][y - 1] == '1' && !vis[x][y - 1] {
            Solution::dfs(grid, x, y - 1, vis);
        }
        if y + 1 < grid[0].len() && grid[x][y + 1] == '1' && !vis[x][y + 1] {
            Solution::dfs(grid, x, y + 1, vis);
        }
    }

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }
        let m = grid.len();
        let n = grid[0].len();
        let mut vis = vec![vec![false; n]; m];
        let mut ans = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] != '1' || vis[i][j] {
                    continue;
                }
                Solution::dfs(&grid, i, j, &mut vis);
                ans += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_islands() {
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0']
            ]),
            1
        );
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1']
            ]),
            3
        );
    }
}
