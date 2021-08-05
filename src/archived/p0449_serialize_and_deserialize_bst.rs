#![allow(dead_code)]
pub struct Solution;

use crate::util::TreeNode;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Codec {}

impl Codec {
    fn new() -> Self {
        Codec {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut s = String::new();
        let mut q = VecDeque::new();
        if let Some(node) = root {
            s.push_str(node.borrow().val.to_string().as_str());
            q.push_back(node);
        } else {
            return "".to_owned();
        }
        while let Some(node) = q.pop_front() {
            s.push(',');
            if let Some(left) = &node.borrow().left {
                s.push_str(left.borrow().val.to_string().as_str());
                q.push_back(left.clone());
            } else {
                s.push('x');
            }
            s.push(',');
            if let Some(right) = &node.borrow().right {
                s.push_str(right.borrow().val.to_string().as_str());
                q.push_back(right.clone());
            } else {
                s.push('x');
            }
        }
        s
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            return None;
        }
        let mut nodes: VecDeque<_> = data
            .split(',')
            .map(|v| {
                v.parse::<i32>()
                    .ok()
                    .map(|v| Rc::new(RefCell::new(TreeNode::new(v))))
            })
            .collect();
        let root = nodes.pop_front().unwrap();
        let mut q = VecDeque::new();
        q.push_back(root.as_ref().unwrap().clone());
        while let Some(parent) = q.pop_front() {
            if let Some(Some(v)) = nodes.pop_front() {
                parent.borrow_mut().left = Some(v.clone());
                q.push_back(v);
            }
            if let Some(Some(v)) = nodes.pop_front() {
                parent.borrow_mut().right = Some(v.clone());
                q.push_back(v);
            }
        }
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codec() {
        let codec = Codec::new();
        assert_eq!(
            codec.deserialize(codec.serialize(tree!(2, 1, 3))),
            tree!(2, 1, 3)
        );
        assert_eq!(codec.deserialize(codec.serialize(tree!())), tree!());
    }
}
