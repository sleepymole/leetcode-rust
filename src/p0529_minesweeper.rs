#![allow(dead_code)]
pub struct Solution {}

use std::char;
use std::collections::{HashSet, VecDeque};

impl Solution {
    fn step_adjacent<F>(x: usize, y: usize, board: &Vec<Vec<char>>, mut f: F)
    where
        F: FnMut(usize, usize, char),
    {
        let (m, n) = (board.len(), board[0].len());
        for i in (x as i32 - 1)..=(x as i32 + 1) {
            for j in (y as i32 - 1)..=(y as i32 + 1) {
                if i >= 0 && i < m as i32 && j >= 0 && j < n as i32 {
                    f(i as usize, j as usize, board[i as usize][j as usize])
                }
            }
        }
    }

    fn get_mines(x: usize, y: usize, board: &Vec<Vec<char>>) -> i32 {
        let mut mines = 0;
        Solution::step_adjacent(x, y, board, |_, _, c| {
            if c == 'M' || c == 'X' {
                mines += 1;
            }
        });
        mines
    }

    pub fn update_board(board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
        let mut board = board;
        let (x, y) = (click[0] as usize, click[1] as usize);
        if board[x][y] == 'M' {
            board[x][y] = 'X';
            return board;
        }
        let mut clicked = HashSet::new();
        let mines = Solution::get_mines(x, y, &board);
        if mines > 0 {
            board[x][y] = char::from_u32('0' as u32 + mines as u32).unwrap();
            return board;
        }
        let mut q = VecDeque::new();
        q.push_back((x, y));
        clicked.insert((x, y));
        while let Some((i, j)) = q.pop_front() {
            let mines = Solution::get_mines(i, j, &board);
            if mines > 0 {
                board[i][j] = char::from_u32('0' as u32 + mines as u32).unwrap();
                continue;
            };
            board[i][j] = 'B';
            Solution::step_adjacent(i, j, &board, |x, y, c| {
                if c == 'E' && !clicked.contains(&(x, y)) {
                    q.push_back((x, y));
                    clicked.insert((x, y));
                }
            });
        }
        board
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_board() {
        assert_eq!(
            Solution::update_board(
                vec![
                    vec!['E', 'E', 'E', 'E', 'E'],
                    vec!['E', 'E', 'M', 'E', 'E'],
                    vec!['E', 'E', 'E', 'E', 'E'],
                    vec!['E', 'E', 'E', 'E', 'E']
                ],
                vec![3, 0]
            ),
            vec![
                vec!['B', '1', 'E', '1', 'B'],
                vec!['B', '1', 'M', '1', 'B'],
                vec!['B', '1', '1', '1', 'B'],
                vec!['B', 'B', 'B', 'B', 'B']
            ]
        );
        assert_eq!(
            Solution::update_board(
                vec![
                    vec!['B', '1', 'E', '1', 'B'],
                    vec!['B', '1', 'M', '1', 'B'],
                    vec!['B', '1', '1', '1', 'B'],
                    vec!['B', 'B', 'B', 'B', 'B']
                ],
                vec![1, 2]
            ),
            vec![
                vec!['B', '1', 'E', '1', 'B'],
                vec!['B', '1', 'X', '1', 'B'],
                vec!['B', '1', '1', '1', 'B'],
                vec!['B', 'B', 'B', 'B', 'B']
            ]
        );
    }
}
