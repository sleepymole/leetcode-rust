#![allow(dead_code)]
pub struct Solution {}

use crate::util::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

enum Res {
    Lca(Option<Rc<RefCell<TreeNode>>>),
    Child(i32),
}

impl Solution {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, pval: i32, qval: i32) -> Res {
        if root.is_none() {
            return Res::Child(0);
        }
        let mut count = 0;
        match Solution::dfs(root.as_ref().unwrap().borrow().left.clone(), pval, qval) {
            Res::Lca(lca) => return Res::Lca(lca),
            Res::Child(n) => count += n,
        }
        match Solution::dfs(root.as_ref().unwrap().borrow().right.clone(), pval, qval) {
            Res::Lca(lca) => return Res::Lca(lca),
            Res::Child(n) => count += n,
        }
        let root_val = root.as_ref().unwrap().borrow().val;
        if root_val == pval || root_val == qval {
            count += 1;
        }
        if count == 2 {
            Res::Lca(root)
        } else {
            Res::Child(count)
        }
    }

    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Res::Lca(lca) = Solution::dfs(
            root,
            p.as_ref().unwrap().borrow().val,
            q.as_ref().unwrap().borrow().val,
        ) {
            lca
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowest_common_ancestor() {
        assert_eq!(
            Solution::lowest_common_ancestor(
                tree!(3, 5, 1, 6, 2, 0, 8, null, null, 7, 4),
                tree!(5, 6, 2, null, null, 7, 4),
                tree!(1, 0, 8)
            ),
            tree!(3, 5, 1, 6, 2, 0, 8, null, null, 7, 4)
        );
        assert_eq!(
            Solution::lowest_common_ancestor(
                tree!(3, 5, 1, 6, 2, 0, 8, null, null, 7, 4),
                tree!(5, 6, 2, null, null, 7, 4),
                tree!(4)
            ),
            tree!(5, 6, 2, null, null, 7, 4)
        );
        assert_eq!(
            Solution::lowest_common_ancestor(tree!(1, 2), tree!(1, 2), tree!(2)),
            tree!(1, 2)
        );
    }
}
