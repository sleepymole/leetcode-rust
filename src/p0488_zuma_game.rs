#![allow(dead_code)]
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    fn search_min_step(
        mut board: Vec<char>,
        hand: &Vec<char>,
        used: i32,
        cache: &mut HashMap<(Vec<char>, i32), i32>,
    ) -> i32 {
        loop {
            let mut n = 0;
            let mut i = 0;
            while i < board.len() {
                let mut j = i + 1;
                while j < board.len() && board[j] == board[i] {
                    j += 1;
                }
                if j - i >= 3 {
                    i = j;
                    continue;
                }
                for k in i..j {
                    board[n] = board[k];
                    n += 1;
                }
                i = j;
            }
            if n == board.len() {
                break;
            }
            board.truncate(n);
        }
        if let Some(&res) = cache.get(&(board.clone(), used)) {
            return res;
        }
        if board.is_empty() {
            return 0;
        }
        let mut min_step = -1;
        for u in 0..hand.len() {
            if (1 << u) & used > 0 {
                continue;
            }
            let c = hand[u];
            for i in 0..board.len() {
                let mut v: Vec<char> = Vec::from(&board[0..i]);
                v.push(c);
                for j in i..board.len() {
                    v.push(board[j])
                }
                let step = Solution::search_min_step(v, &hand, used | (1 << u), cache);
                if step != -1 {
                    if min_step == -1 {
                        min_step = step + 1;
                    } else {
                        min_step = min_step.min(step + 1);
                    }
                }
            }
        }
        cache.insert((board, used), min_step);
        min_step
    }

    pub fn find_min_step(board: String, hand: String) -> i32 {
        let board: Vec<char> = board.chars().collect();
        let hand: Vec<char> = hand.chars().collect();
        let mut cache = HashMap::new();
        Solution::search_min_step(board, &hand, 0, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min_step() {
        assert_eq!(
            Solution::find_min_step("WRRBBW".to_owned(), "RB".to_owned()),
            -1
        );
        assert_eq!(
            Solution::find_min_step("WWRRBBWW".to_owned(), "WRBRW".to_owned()),
            2
        );
        assert_eq!(
            Solution::find_min_step("G".to_owned(), "GGGGG".to_owned()),
            2
        );
        assert_eq!(
            Solution::find_min_step("RBYYBBRRB".to_owned(), "YRBGB".to_owned()),
            3
        );
        assert_eq!(
            Solution::find_min_step("RRWWRRBBRR".to_owned(), "WB".to_owned()),
            2
        );
    }
}
