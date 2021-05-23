#![allow(dead_code)]
pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    fn bfs(heights: &mut Vec<Vec<i32>>, flag: i32, mask: i32, top_left: bool) {
        let m = heights.len();
        let n = heights[0].len();
        let mut q = VecDeque::new();
        if top_left {
            for j in 0..n {
                heights[0][j] |= flag;
                q.push_back((0, j));
            }
            for i in 1..m {
                heights[i][0] |= flag;
                q.push_back((i, 0));
            }
        } else {
            for j in 0..n {
                heights[m - 1][j] |= flag;
                q.push_back((m - 1, j));
            }
            for i in 0..m - 1 {
                heights[i][n - 1] |= flag;
                q.push_back((i, n - 1));
            }
        }
        while let Some((x, y)) = q.pop_front() {
            if x > 0
                && (heights[x - 1][y] & flag == 0)
                && heights[x - 1][y] & mask >= (heights[x][y] & mask)
            {
                heights[x - 1][y] |= flag;
                q.push_back((x - 1, y));
            }
            if x + 1 < m
                && (heights[x + 1][y] & flag == 0)
                && heights[x + 1][y] & mask >= (heights[x][y] & mask)
            {
                heights[x + 1][y] |= flag;
                q.push_back((x + 1, y));
            }
            if y > 0
                && (heights[x][y - 1] & flag == 0)
                && heights[x][y - 1] & mask >= (heights[x][y] & mask)
            {
                heights[x][y - 1] |= flag;
                q.push_back((x, y - 1));
            }
            if y + 1 < n
                && (heights[x][y + 1] & flag == 0)
                && heights[x][y + 1] & mask >= (heights[x][y] & mask)
            {
                heights[x][y + 1] |= flag;
                q.push_back((x, y + 1));
            }
        }
    }

    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut heights = heights;
        Solution::bfs(&mut heights, 1 << 29, 0x00ffffff, true);
        Solution::bfs(&mut heights, 1 << 30, 0x00ffffff, false);
        let mut res = Vec::new();
        let m = heights.len();
        let n = heights[0].len();
        for i in 0..m {
            for j in 0..n {
                if heights[i][j] & (1 << 29) > 0 && heights[i][j] & (1 << 30) > 0 {
                    res.push(vec![i as i32, j as i32]);
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pacific_atlantic() {
        assert_eq!(
            Solution::pacific_atlantic(vec![
                vec![1, 2, 2, 3, 5],
                vec![3, 2, 3, 4, 4],
                vec![2, 4, 5, 3, 1],
                vec![6, 7, 1, 4, 5],
                vec![5, 1, 1, 2, 4]
            ]),
            vec![
                vec![0, 4],
                vec![1, 3],
                vec![1, 4],
                vec![2, 2],
                vec![3, 0],
                vec![3, 1],
                vec![4, 0]
            ]
        );
        assert_eq!(
            Solution::pacific_atlantic(vec![vec![2, 1], vec![1, 2]]),
            vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 1]]
        );
    }
}
