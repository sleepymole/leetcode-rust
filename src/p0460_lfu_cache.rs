#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
struct Node {
    key: i32,
    val: i32,
    freq: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: i32, val: i32) -> Self {
        Node {
            key,
            val,
            freq: 0,
            prev: None,
            next: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct FreqNode {
    tail: Option<Rc<RefCell<Node>>>,
}

impl FreqNode {
    fn new() -> Self {
        let tail = Rc::new(RefCell::new(Node::new(0, 0)));
        tail.borrow_mut().next = Some(tail.clone());
        tail.borrow_mut().prev = Some(tail.clone());
        FreqNode { tail: Some(tail) }
    }

    fn push(&mut self, node: Rc<RefCell<Node>>) {
        node.borrow_mut().next = self.tail.clone();
        node.borrow_mut().prev = self.tail.as_ref().unwrap().borrow().prev.clone();
        let tail_prev = self.tail.as_ref().unwrap().borrow().prev.clone();
        tail_prev.as_ref().unwrap().borrow_mut().next = Some(node.clone());
        self.tail.as_ref().unwrap().borrow_mut().prev = Some(node);
    }

    fn pop(&mut self) -> Option<Rc<RefCell<Node>>> {
        if self.is_empty() {
            return None;
        }
        let head = self.tail.as_ref().unwrap().borrow().next.clone();
        self.remove(head.as_ref().unwrap().clone());
        head
    }

    fn remove(&mut self, node: Rc<RefCell<Node>>) {
        let prev = node.borrow_mut().prev.take();
        let next = node.borrow_mut().next.take();
        prev.as_ref().unwrap().borrow_mut().next = next.clone();
        next.as_ref().unwrap().borrow_mut().prev = prev;
    }

    fn is_empty(&self) -> bool {
        let head = &self.tail.as_ref().unwrap().borrow().next;
        Rc::ptr_eq(head.as_ref().unwrap(), self.tail.as_ref().unwrap())
    }
}

#[derive(Debug, Default)]
struct LFUCache {
    capacity: usize,
    nodes: HashMap<i32, Rc<RefCell<Node>>>,
    freqs: HashMap<i32, Rc<RefCell<FreqNode>>>,
    min_freq: i32,
}

impl LFUCache {
    fn new(capacity: i32) -> Self {
        LFUCache {
            capacity: capacity as usize,
            nodes: HashMap::new(),
            freqs: HashMap::new(),
            min_freq: 0,
        }
    }

    fn put(&mut self, key: i32, val: i32) {
        if let Some(node) = self.nodes.get_mut(&key) {
            node.borrow_mut().val = val;
            let node = node.clone();
            self.on_hit(node);
        } else {
            self.put_new(key, val);
        };
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.nodes.get_mut(&key) {
            let val = node.borrow().val;
            let node = node.clone();
            self.on_hit(node);
            val
        } else {
            -1
        }
    }

    fn on_hit(&mut self, node: Rc<RefCell<Node>>) {
        let mut freq = node.borrow().freq;
        if let Some(freq_node) = self.freqs.get_mut(&freq) {
            freq_node.borrow_mut().remove(node.clone());
            if freq_node.borrow().is_empty() {
                self.freqs.remove(&freq);
                if freq == self.min_freq {
                    self.min_freq += 1;
                }
            }
        }
        freq += 1;
        node.borrow_mut().freq = freq;
        let freq_node = self
            .freqs
            .entry(freq)
            .or_insert_with(|| Rc::new(RefCell::new(FreqNode::new())));
        freq_node.borrow_mut().push(node);
    }

    fn put_new(&mut self, key: i32, val: i32) {
        if self.capacity == 0 {
            return;
        }
        if self.nodes.len() >= self.capacity {
            if let Some(freq_node) = self.freqs.get_mut(&self.min_freq) {
                let evicted = freq_node.borrow_mut().pop().as_ref().unwrap().borrow().key;
                self.nodes.remove(&evicted);
                if freq_node.borrow().is_empty() {
                    self.freqs.remove(&self.min_freq);
                }
            }
        }
        self.min_freq = 0;
        let freq_node = self
            .freqs
            .entry(0)
            .or_insert_with(|| Rc::new(RefCell::new(FreqNode::new())));
        let node = Rc::new(RefCell::new(Node::new(key, val)));
        self.nodes.insert(key, node.clone());
        freq_node.borrow_mut().push(node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lfu_cache() {
        let mut lfu = LFUCache::new(2);
        lfu.put(1, 1);
        lfu.put(2, 2);
        assert_eq!(lfu.get(1), 1);
        lfu.put(3, 3);
        assert_eq!(lfu.get(2), -1);
        assert_eq!(lfu.get(3), 3);
        lfu.put(4, 4);
        assert_eq!(lfu.get(1), -1);
        assert_eq!(lfu.get(3), 3);
        assert_eq!(lfu.get(4), 4);
    }
}
