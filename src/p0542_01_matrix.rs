#![allow(dead_code)]
pub struct Solution {}

use std::collections::VecDeque;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (mat.len(), mat[0].len());
        let mut dist = vec![vec![-1; n]; m];
        let mut q = VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 0 {
                    dist[i][j] = 0;
                    q.push_back((i, j));
                }
            }
        }
        let mut steps = 1;
        while !q.is_empty() {
            let l = q.len();
            for _ in 0..l {
                if let Some((x, y)) = q.pop_front() {
                    if x > 0 && dist[x - 1][y] == -1 {
                        dist[x - 1][y] = steps;
                        q.push_back((x - 1, y));
                    }
                    if x + 1 < m && dist[x + 1][y] == -1 {
                        dist[x + 1][y] = steps;
                        q.push_back((x + 1, y));
                    }
                    if y > 0 && dist[x][y - 1] == -1 {
                        dist[x][y - 1] = steps;
                        q.push_back((x, y - 1));
                    }
                    if y + 1 < n && dist[x][y + 1] == -1 {
                        dist[x][y + 1] = steps;
                        q.push_back((x, y + 1));
                    }
                }
            }
            steps += 1;
        }
        dist
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_matrix() {
        assert_eq!(
            Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]
        );
        assert_eq!(
            Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]]
        );
    }
}
