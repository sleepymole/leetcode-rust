#![allow(dead_code)]
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct LRUEntry {
    key: i32,
    val: i32,
    prev: Option<Rc<RefCell<LRUEntry>>>,
    next: Option<Rc<RefCell<LRUEntry>>>,
}

impl LRUEntry {
    fn new(key: i32, val: i32) -> Self {
        LRUEntry {
            key,
            val,
            prev: None,
            next: None,
        }
    }
}

struct LRUCache {
    cap: usize,
    map: HashMap<i32, Rc<RefCell<LRUEntry>>>,
    head: Option<Rc<RefCell<LRUEntry>>>,
    tail: Option<Rc<RefCell<LRUEntry>>>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        LRUCache {
            cap: capacity as usize,
            map: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(entry) = self.map.get(&key) {
            let val = entry.borrow().val;
            let entry = entry.clone();
            self.remove(entry.clone());
            self.push(entry);
            return val;
        }
        -1
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(entry) = self.map.get_mut(&key) {
            entry.borrow_mut().val = value;
            let entry = entry.clone();
            self.remove(entry.clone());
            self.push(entry);
        } else {
            if self.map.len() == self.cap {
                let entry = self.pop();
                self.map.remove(&entry.borrow().key);
            }
            let entry = Rc::new(RefCell::new(LRUEntry::new(key, value)));
            self.map.insert(key, entry.clone());
            self.push(entry);
        }
    }

    fn push(&mut self, entry: Rc<RefCell<LRUEntry>>) {
        if self.head.is_none() {
            self.head = Some(entry);
            self.tail = self.head.clone();
        } else {
            self.head.as_ref().unwrap().borrow_mut().prev = Some(entry.clone());
            entry.borrow_mut().next = self.head.take();
            self.head = Some(entry);
        }
    }

    fn pop(&mut self) -> Rc<RefCell<LRUEntry>> {
        let entry = self.tail.as_ref().unwrap().clone();
        self.remove(entry.clone());
        entry
    }

    fn remove(&mut self, entry: Rc<RefCell<LRUEntry>>) {
        if Rc::ptr_eq(self.head.as_ref().unwrap(), &entry) {
            self.head = entry.borrow().next.clone();
            if let Some(entry) = self.head.as_mut() {
                entry.borrow_mut().prev = None;
            } else {
                self.tail = None;
            }
        } else if Rc::ptr_eq(self.tail.as_ref().unwrap(), &entry) {
            self.tail = entry.borrow().prev.clone();
            self.tail.as_ref().unwrap().borrow_mut().next = None;
        } else {
            entry.borrow().prev.as_ref().unwrap().borrow_mut().next = entry.borrow().next.clone();
            entry.borrow().next.as_ref().unwrap().borrow_mut().prev = entry.borrow().prev.clone();
        }
        entry.borrow_mut().prev = None;
        entry.borrow_mut().next = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 0);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 0);
        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }
}
