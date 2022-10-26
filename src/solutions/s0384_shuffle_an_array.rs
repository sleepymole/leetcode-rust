#![allow(dead_code)]

struct Solution {
    nums: Vec<i32>,
    seed: usize,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Solution { nums, seed: 0 }
    }

    fn reset(&self) -> Vec<i32> {
        self.nums.clone()
    }

    fn rand(&mut self, n: usize) -> usize {
        self.seed = (self.seed * 88054201 + 367) % 1000000007;
        self.seed % n
    }

    fn shuffle(&mut self) -> Vec<i32> {
        let mut nums = self.nums.clone();
        for i in 0..nums.len() {
            let j = self.rand(self.nums.len() - i);
            nums.swap(i, i + j);
        }
        nums
    }
}
