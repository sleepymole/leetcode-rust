#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn dfs(
        root: Option<Rc<RefCell<TreeNode>>>,
        sum: i32,
        path: &mut Vec<i32>,
        paths: &mut Vec<Vec<i32>>,
    ) {
        let root_val = root.as_ref().unwrap().borrow().val;
        let sum = sum - root_val;
        path.push(root_val);
        println!("{root_val} {path:?}");
        let left = root.as_ref().unwrap().borrow().left.clone();
        let right = root.as_ref().unwrap().borrow().right.clone();
        if left.is_none() && right.is_none() {
            if sum == 0 {
                paths.push(path.clone());
            }
            path.pop();
            return;
        }
        if left.is_some() {
            Solution::dfs(left, sum, path, paths)
        }
        if right.is_some() {
            Solution::dfs(right, sum, path, paths)
        }
        path.pop();
    }

    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }
        let mut path = Vec::new();
        let mut paths = Vec::new();
        Solution::dfs(root, sum, &mut path, &mut paths);
        paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_path_sum() {
        assert_eq!(
            Solution::path_sum(tree!(5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1), 22),
            vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]]
        );
    }
}
