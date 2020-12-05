#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            let mut flag = 0;
            for j in 0..9 {
                if board[i][j] == '.' {
                    continue;
                }
                let d = board[i][j] as i32 - '0' as i32;
                if flag & (1 << d) != 0 {
                    return false;
                }
                flag |= 1 << d;
            }
            flag = 0;
            for j in 0..9 {
                if board[j][i] == '.' {
                    continue;
                }
                let d = board[j][i] as i32 - '0' as i32;
                if flag & (1 << d) != 0 {
                    return false;
                }
                flag |= 1 << d;
            }
        }
        for x in (0..9).step_by(3) {
            for y in (0..9).step_by(3) {
                let mut flag = 0;
                for i in x..x + 3 {
                    for j in y..y + 3 {
                        if board[i][j] == '.' {
                            continue;
                        }
                        let d = board[i][j] as i32 - '0' as i32;
                        if flag & (1 << d) != 0 {
                            return false;
                        }
                        flag |= 1 << d;
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_sudoku() {
        assert_eq!(
            Solution::is_valid_sudoku(vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ]),
            false
        );
        assert_eq!(
            Solution::is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            true
        );
    }
}
