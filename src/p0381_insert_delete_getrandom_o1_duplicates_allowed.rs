#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct RandomizedCollection {
    nums: Vec<i32>,
    m: HashMap<i32, HashSet<usize>>,
    seed: usize,
}

impl RandomizedCollection {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, val: i32) -> bool {
        let s = self.m.entry(val).or_insert_with(HashSet::new);
        s.insert(self.nums.len());
        self.nums.push(val);
        s.len() == 1
    }

    fn remove(&mut self, val: i32) -> bool {
        let s = self.m.entry(val).or_insert_with(HashSet::new);
        if let Some(&i) = s.iter().next() {
            s.remove(&i);
            if s.is_empty() {
                self.m.remove(&val);
            }
            self.nums.swap_remove(i);
            if i < self.nums.len() {
                let s = self.m.entry(self.nums[i]).or_insert_with(HashSet::new);
                s.remove(&self.nums.len());
                s.insert(i);
            }
            true
        } else {
            false
        }
    }

    fn get_random(&mut self) -> i32 {
        self.seed = (self.seed * 88054201 + 367) % 1000000007;
        self.nums[self.seed % self.nums.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_randomized_collection() {
        let mut c = RandomizedCollection::new();
        assert!(c.insert(9));
        assert!(!c.insert(9));
        assert!(c.insert(1));
        assert!(!c.insert(1));
        assert!(c.insert(2));
        assert!(!c.insert(1));
        assert!(c.remove(2));
        assert!(c.remove(1));
        assert!(c.remove(1));
        assert!(!c.insert(9));
        assert!(c.remove(1));
    }
}
