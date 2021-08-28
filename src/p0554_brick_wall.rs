#![allow(dead_code)]
pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let mut m = HashMap::new();
        for w in &wall {
            let mut l = w[0];
            for i in 1..w.len() {
                *m.entry(l).or_insert(0) += 1;
                l += w[i];
            }
        }
        wall.len() as i32 - m.values().max().cloned().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_least_bricks() {
        assert_eq!(
            Solution::least_bricks(vec![
                vec![1, 2, 2, 1],
                vec![3, 1, 2],
                vec![1, 3, 2],
                vec![2, 4],
                vec![3, 1, 2],
                vec![1, 3, 1, 1]
            ]),
            2
        );
        assert_eq!(Solution::least_bricks(vec![vec![1], vec![1], vec![1]]), 3);
    }
}
