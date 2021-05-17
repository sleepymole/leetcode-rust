#![allow(dead_code)]
pub struct Solution;

impl Solution {
    fn dfs(
        board: &Vec<Vec<char>>,
        word: &Vec<char>,
        matched: usize,
        vis: &mut Vec<Vec<bool>>,
        x: usize,
        y: usize,
    ) -> bool {
        if matched >= word.len() {
            return true;
        }
        let m = board.len();
        let n = board[0].len();
        vis[x][y] = true;
        if x > 0
            && !vis[x - 1][y]
            && board[x - 1][y] == word[matched]
            && Solution::dfs(board, word, matched + 1, vis, x - 1, y)
        {
            return true;
        }
        if y > 0
            && !vis[x][y - 1]
            && board[x][y - 1] == word[matched]
            && Solution::dfs(board, word, matched + 1, vis, x, y - 1)
        {
            return true;
        }
        if x + 1 < m
            && !vis[x + 1][y]
            && board[x + 1][y] == word[matched]
            && Solution::dfs(board, word, matched + 1, vis, x + 1, y)
        {
            return true;
        }
        if y + 1 < n
            && !vis[x][y + 1]
            && board[x][y + 1] == word[matched]
            && Solution::dfs(board, word, matched + 1, vis, x, y + 1)
        {
            return true;
        }
        vis[x][y] = false;
        false
    }

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if word.is_empty() {
            return true;
        }
        let m = board.len();
        let n = board[0].len();
        let mut vis: Vec<Vec<bool>> = Vec::new();
        vis.resize(m, vec![]);
        for l in vis.iter_mut() {
            l.resize(n, false);
        }
        let word: Vec<char> = word.chars().collect();
        for i in 0..m {
            for j in 0..n {
                if board[i][j] == word[0] && Solution::dfs(&board, &word, 1, &mut vis, i, j) {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exist() {
        assert!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCCED".to_owned()
            ));
        assert!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "SEE".to_owned()
            ));
        assert!(!
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                "ABCB".to_owned()
            ));
    }
}
