#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }
        let m = grid.len();
        let n = grid[0].len();
        let mut count = 0;
        let mut connected = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    count += 1;
                    if i > 0 && grid[i - 1][j] == 1 {
                        connected += 1;
                    }
                    if i + 1 < m && grid[i + 1][j] == 1 {
                        connected += 1;
                    }
                    if j > 0 && grid[i][j - 1] == 1 {
                        connected += 1;
                    }
                    if j + 1 < n && grid[i][j + 1] == 1 {
                        connected += 1;
                    }
                }
            }
        }
        count * 4 - connected
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_island_perimeter() {
        assert_eq!(
            Solution::island_perimeter(vec![
                vec![0, 1, 0, 0],
                vec![1, 1, 1, 0],
                vec![0, 1, 0, 0],
                vec![1, 1, 0, 0]
            ]),
            16
        );
        assert_eq!(Solution::island_perimeter(vec![vec![1]]), 4);
        assert_eq!(Solution::island_perimeter(vec![vec![1, 0]]), 4);
    }
}
