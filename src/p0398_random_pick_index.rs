#![allow(dead_code)]

use std::collections::HashMap;

struct Solution {
    m: HashMap<i32, Vec<i32>>,
    seed: usize,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        let mut m = HashMap::new();
        for i in 0..nums.len() {
            m.entry(nums[i]).or_insert_with(Vec::new).push(i as i32);
        }
        Solution { m, seed: 0 }
    }

    fn pick(&mut self, target: i32) -> i32 {
        self.seed = (self.seed * 88054201 + 367) % 1000000007;
        if let Some(v) = self.m.get(&target) {
            v[self.seed % v.len()]
        } else {
            0
        }
    }
}
