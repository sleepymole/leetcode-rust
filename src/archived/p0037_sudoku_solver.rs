#![allow(dead_code)]
pub struct Solution;

use std::char;

impl Solution {
    fn dfs(board: &mut Vec<Vec<char>>, pos: usize) -> bool {
        if pos == 81 {
            return true;
        }
        let (x, y) = (pos / 9, pos % 9);
        if board[x][y] != '.' {
            return Solution::dfs(board, pos + 1);
        }
        let mut flag = 0;
        for i in 0..9 {
            if board[x][i] != '.' {
                flag |= 1 << (board[x][i] as i32 - '0' as i32);
            }
            if board[i][y] != '.' {
                flag |= 1 << (board[i][y] as i32 - '0' as i32);
            }
        }
        let (x0, y0) = (x / 3 * 3, y / 3 * 3);
        for i in x0..x0 + 3 {
            for j in y0..y0 + 3 {
                if board[i][j] != '.' {
                    flag |= 1 << (board[i][j] as i32 - '0' as i32);
                }
            }
        }
        for i in 1..=9 {
            let c = char::from_digit(i, 10).unwrap();
            if flag & (1 << (c as i32 - '0' as i32)) != 0 {
                continue;
            }
            board[x][y] = c;
            if Solution::dfs(board, pos + 1) {
                return true;
            }
        }
        board[x][y] = '.';
        false
    }

    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Solution::dfs(board, 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::archived::p0036_valid_sudoku;

    #[test]
    fn test_solve_sudoku() {
        let board = vec![
            vec!['5', '3', '.', '.', '.', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let mut board2 = board.clone();
        Solution::solve_sudoku(&mut board2);
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != '.' {
                    assert_eq!(board[i][j], board2[i][j]);
                }
            }
        }
        for l in board2.iter() {
            println!("{:?}", l);
        }
        assert!(p0036_valid_sudoku::Solution::is_valid_sudoku(board2));
    }
}
