#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let mut board = board;
        let m = board.len();
        let n = board[0].len();
        let mut count = 0;
        for i in 0..m {
            for j in 0..n {
                if board[i][j] != 'X' {
                    continue;
                }
                count += 1;
                for k in i..m {
                    if board[k][j] != 'X' {
                        break;
                    }
                    board[k][j] = 'Y';
                }
                for k in j + 1..n {
                    if board[i][k] != 'X' {
                        break;
                    }
                    board[i][k] = 'Y';
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_battleships() {
        assert_eq!(
            Solution::count_battleships(vec![
                vec!['X', '.', '.', 'X'],
                vec!['.', '.', '.', 'X'],
                vec!['.', '.', '.', 'X']
            ]),
            2
        );
        assert_eq!(Solution::count_battleships(vec![vec!['.']]), 0);
    }
}
