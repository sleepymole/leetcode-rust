#![allow(dead_code)]
pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn can_i_win(m: i32, n: i32) -> bool {
        let score = |state| {
            let mut s = 0;
            for i in 0..m {
                if state & (1 << i) > 0 {
                    s += i + 1
                }
            }
            s
        };

        let mut first = 0;
        for i in 0..m {
            first |= 1 << i;
        }
        if score(first) < n {
            return false;
        }

        let mut wins = vec![false; 1 << m];
        let mut vis = vec![false; 1 << m];
        wins[first] = true;
        let mut q = VecDeque::new();
        q.push_back(first);
        vis[first] = true;

        while !q.is_empty() {
            let l = q.len();
            for _ in 0..l {
                let state = q.pop_front().unwrap();
                let s = score(state);
                for i in 0..m {
                    if state & (1 << i) == 0 {
                        continue;
                    }
                    let next = state ^ (1 << i);
                    wins[next] |= (s >= n) | !wins[state];
                    if !vis[next] {
                        q.push_back(next);
                        vis[next] = true;
                    }
                }
            }
        }
        wins[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_i_win() {
        assert!(!Solution::can_i_win(10, 11));
        assert!(Solution::can_i_win(10, 0));
        assert!(Solution::can_i_win(10, 1));
        assert!(!Solution::can_i_win(5, 50));
    }
}
