#![allow(dead_code)]
pub struct Solution {}

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, path: &mut Vec<i32>, paths: &mut Vec<String>) {
        let node = root.as_ref().unwrap().borrow();
        if node.left.is_none() && node.right.is_none() {
            paths.push(
                path.iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join("->"),
            );
            return;
        }
        if node.left.is_some() {
            path.push(node.left.as_ref().unwrap().borrow().val);
            Solution::dfs(node.left.clone(), path, paths);
            path.pop();
        }
        if node.right.is_some() {
            path.push(node.right.as_ref().unwrap().borrow().val);
            Solution::dfs(node.right.clone(), path, paths);
            path.pop();
        }
    }

    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut paths = vec![];
        if let Some(node) = &root {
            let mut path = vec![node.borrow().val];
            Solution::dfs(root, &mut path, &mut paths);
        }
        paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_tree_paths() {
        let mut paths = Solution::binary_tree_paths(tree!(1, 2, 3, null, 5));
        paths.sort_unstable();
        assert_eq!(paths, vec!["1->2->5", "1->3"]);
    }
}
