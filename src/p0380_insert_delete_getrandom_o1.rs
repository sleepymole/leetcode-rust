#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Default)]
struct RandomizedSet {
    nums: Vec<i32>,
    m: HashMap<i32, usize>,
    seed: usize,
}

impl RandomizedSet {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.m.contains_key(&val) {
            return false;
        }
        self.m.insert(val, self.nums.len());
        self.nums.push(val);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(i) = self.m.remove(&val) {
            self.nums.swap_remove(i);
            if i < self.nums.len() {
                self.m.insert(self.nums[i], i);
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
