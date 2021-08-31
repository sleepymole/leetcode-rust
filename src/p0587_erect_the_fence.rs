#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut trees = trees;
        trees.sort_unstable();
        let mut stk: Vec<usize> = vec![0];
        let mut used = vec![false; trees.len()];
        for i in (1..trees.len()).chain((0..trees.len() - 1).rev()) {
            if used[i] {
                continue;
            }
            while stk.len() >= 2 {
                let (a, b, c) = (stk[stk.len() - 2], stk[stk.len() - 1], i);
                let (x1, y1) = (trees[a][0] - trees[b][0], trees[a][1] - trees[b][1]);
                let (x2, y2) = (trees[b][0] - trees[c][0], trees[b][1] - trees[c][1]);
                if x1 * y2 - x2 * y1 <= 0 {
                    break;
                }
                stk.pop();
                used[b] = false;
            }
            used[i] = true;
            stk.push(i);
        }
        if stk.len() > 1 {
            stk.pop();
        }
        let mut points = Vec::new();
        for i in stk {
            points.push(trees[i].clone());
        }
        points
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outer_trees() {
        assert_eq!(
            Solution::outer_trees(vec![
                vec![1, 1],
                vec![2, 2],
                vec![2, 0],
                vec![2, 4],
                vec![3, 3],
                vec![4, 2]
            ]),
            vec![vec![1, 1], vec![2, 4], vec![3, 3], vec![4, 2], vec![2, 0]]
        );
        assert_eq!(
            Solution::outer_trees(vec![vec![1, 2], vec![2, 2], vec![4, 2]]),
            vec![vec![1, 2], vec![2, 2], vec![4, 2]]
        );
    }
}
