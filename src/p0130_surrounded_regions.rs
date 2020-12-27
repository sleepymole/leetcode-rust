#![allow(dead_code)]
pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let m = board.len();
        if m == 0 {
            return;
        }
        let n = board[0].len();
        if n == 0 {
            return;
        }
        let mut q = VecDeque::new();
        for i in 0..m {
            if board[i][0] == 'O' {
                board[i][0] = 'M';
                q.push_back((i, 0));
            }
            if board[i][n - 1] == 'O' {
                board[i][n - 1] = 'M';
                q.push_back((i, n - 1));
            }
        }
        for j in 0..n {
            if board[0][j] == 'O' {
                board[0][j] = 'M';
                q.push_back((0, j));
            }
            if board[m - 1][j] == 'O' {
                board[m - 1][j] = 'M';
                q.push_back((m - 1, j));
            }
        }
        while let Some((x, y)) = q.pop_front() {
            if x > 1 && board[x - 1][y] == 'O' {
                board[x - 1][y] = 'M';
                q.push_back((x - 1, y));
            }
            if x + 1 < m && board[x + 1][y] == 'O' {
                board[x + 1][y] = 'M';
                q.push_back((x + 1, y));
            }
            if y > 1 && board[x][y - 1] == 'O' {
                board[x][y - 1] = 'M';
                q.push_back((x, y - 1));
            }
            if y + 1 < n && board[x][y + 1] == 'O' {
                board[x][y + 1] = 'M';
                q.push_back((x, y + 1));
            }
        }
        for i in 0..m {
            for j in 0..n {
                if board[i][j] == 'M' {
                    board[i][j] = 'O';
                } else if board[i][j] == 'O' {
                    board[i][j] = 'X';
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut board);
        assert_eq!(
            board,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ]
        );
    }
}
