#![allow(dead_code)]
pub struct Solution {}

use std::collections::HashSet;

impl Solution {
    fn dfs(
        x: usize,
        y: usize,
        word: &mut String,
        words: &mut HashSet<String>,
        board: &Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
    ) {
        if !word.is_empty() {
            words.insert(word.clone());
        }
        if word.len() >= 10 {
            return;
        }
        let m = board.len();
        let n = board[0].len();
        if x > 0 && !visited[x - 1][y] {
            visited[x - 1][y] = true;
            word.push(board[x - 1][y]);
            Solution::dfs(x - 1, y, word, words, board, visited);
            word.pop();
            visited[x - 1][y] = false;
        }
        if x + 1 < m && !visited[x + 1][y] {
            visited[x + 1][y] = true;
            word.push(board[x + 1][y]);
            Solution::dfs(x + 1, y, word, words, board, visited);
            word.pop();
            visited[x + 1][y] = false;
        }
        if y > 0 && !visited[x][y - 1] {
            visited[x][y - 1] = true;
            word.push(board[x][y - 1]);
            Solution::dfs(x, y - 1, word, words, board, visited);
            word.pop();
            visited[x][y - 1] = false;
        }
        if y + 1 < n && !visited[x][y + 1] {
            visited[x][y + 1] = true;
            word.push(board[x][y + 1]);
            Solution::dfs(x, y + 1, word, words, board, visited);
            word.pop();
            visited[x][y + 1] = false;
        }
    }

    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let m = board.len();
        let n = board[0].len();
        let mut ws = HashSet::new();
        let mut word = String::new();
        let mut visited = vec![vec![false; n]; m];
        for i in 0..m {
            for j in 0..n {
                visited[i][j] = true;
                word.push(board[i][j]);
                Solution::dfs(i, j, &mut word, &mut ws, &board, &mut visited);
                word.pop();
                visited[i][j] = false;
            }
        }
        let mut ans = Vec::new();
        for w in words {
            if ws.contains(&w) {
                ans.push(w);
            }
        }
        ans.sort_unstable();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_words() {
        assert_eq!(
            Solution::find_words(
                vec![
                    vec!['o', 'a', 'a', 'n'],
                    vec!['e', 't', 'a', 'e'],
                    vec!['i', 'h', 'k', 'r'],
                    vec!['i', 'f', 'l', 'v']
                ],
                vec![
                    "oath".to_owned(),
                    "pea".to_owned(),
                    "eat".to_owned(),
                    "rain".to_owned()
                ]
            ),
            vec!["eat".to_owned(), "oath".to_owned()]
        );
        assert_eq!(
            Solution::find_words(
                vec![vec!['a', 'b'], vec!['c', 'd']],
                vec!["abcb".to_owned()]
            )
            .is_empty(),
            true
        );
    }
}
