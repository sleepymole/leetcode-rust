#![allow(dead_code)]

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
        if let Some(node) = root {
            let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
            let mut nodes = vec![node.borrow().val.to_string()];
            queue.push_back(node);
            while let Some(node) = queue.pop_front() {
                if let Some(left) = node.borrow().left.clone() {
                    nodes.push(left.borrow().val.to_string());
                    queue.push_back(left);
                } else {
                    nodes.push("null".to_owned());
                }
                if let Some(right) = node.borrow().right.clone() {
                    nodes.push(right.borrow().val.to_string());
                    queue.push_back(right);
                } else {
                    nodes.push("null".to_owned());
                }
            }
            nodes.join(",")
        } else {
            "".to_owned()
        }
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes: VecDeque<_> = data
            .split(',')
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|v| {
                v.parse::<i32>()
                    .ok()
                    .map(|v| Rc::new(RefCell::new(TreeNode::new(v))))
            })
            .collect();
        nodes[0].as_ref()?;
        let root = nodes.pop_front().unwrap();
        let mut queue = VecDeque::new();
        queue.push_back(root.as_ref().unwrap().clone());
        while !queue.is_empty() {
            let parent = queue.pop_front().unwrap();
            if let Some(Some(v)) = nodes.pop_front() {
                parent.borrow_mut().left = Some(v.clone());
                queue.push_back(v);
            }
            if let Some(Some(v)) = nodes.pop_front() {
                parent.borrow_mut().right = Some(v.clone());
                queue.push_back(v);
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
        let tree = tree!(1, 2, 3, null, null, 4, 5);
        assert_eq!(codec.deserialize(codec.serialize(tree.clone())), tree);
        let tree = tree!();
        assert_eq!(codec.deserialize(codec.serialize(tree.clone())), tree);
        let tree = tree!(1);
        assert_eq!(codec.deserialize(codec.serialize(tree.clone())), tree);
        let tree = tree!(1, 2);
        assert_eq!(codec.deserialize(codec.serialize(tree.clone())), tree);
    }
}
