#![allow(dead_code)]
pub struct Solution {}

use crate::util::TreeNode;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    fn dfs(root: Rc<RefCell<TreeNode>>, m: &mut HashMap<i32, i32>) -> i32 {
        let mut sum = root.borrow().val;
        if let Some(left) = root.borrow().left.clone() {
            sum += Solution::dfs(left, m);
        }
        if let Some(right) = root.borrow().right.clone() {
            sum += Solution::dfs(right, m);
        }
        *m.entry(sum).or_insert(0) += 1;
        sum
    }

    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = if let Some(root) = root {
            let mut m = HashMap::new();
            Solution::dfs(root, &mut m);
            let max_freq = m.values().max().cloned().unwrap();
            m.into_iter()
                .filter(|&(_, v)| v == max_freq)
                .map(|(k, _)| k)
                .collect()
        } else {
            Vec::new()
        };
        res.sort_unstable();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_frequent_tree_sum() {
        assert_eq!(
            Solution::find_frequent_tree_sum(tree!(5, 2, -3)),
            vec![-3, 2, 4]
        );
        assert_eq!(Solution::find_frequent_tree_sum(tree!(5, 2, -5)), vec![2]);
    }
}
