#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let m = board.len();
        let n = board[0].len();
        for i in 0..m {
            for j in 0..n {
                let mut lives = 0;
                for x in 0..3 {
                    for y in 0..3 {
                        if (x != 1 || y != 1)
                            && (1..=m).contains(&(i + x))
                            && (1..=n).contains(&(j + y))
                            && (board[i + x - 1][j + y - 1] & 1) > 0
                        {
                            lives += 1;
                        }
                    }
                }
                if board[i][j] == 1 && !(2..=3).contains(&lives) || board[i][j] == 0 && lives == 3 {
                    board[i][j] |= 0b10 ^ (board[i][j] << 1);
                } else {
                    board[i][j] |= board[i][j] << 1;
                }
            }
        }
        for i in 0..m {
            for j in 0..n {
                board[i][j] >>= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_of_life() {
        let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        Solution::game_of_life(&mut board);
        assert_eq!(
            board,
            vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]]
        );
        let mut board = vec![vec![1, 1], vec![1, 0]];
        Solution::game_of_life(&mut board);
        assert_eq!(board, vec![vec![1, 1], vec![1, 1]]);
    }
}
