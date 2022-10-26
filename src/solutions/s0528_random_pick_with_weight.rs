#![allow(dead_code)]

struct Solution {
    w: Vec<i32>,
    rng: Rng32,
    sum: i32,
}

impl Solution {
    fn new(w: Vec<i32>) -> Self {
        let sum = w.iter().sum();
        Solution {
            w,
            rng: Rng32::new(),
            sum,
        }
    }

    fn pick_index(&mut self) -> i32 {
        let mut x = (self.rng.next() % self.sum as u32) as i32;
        for i in 0..self.w.len() {
            if x < self.w[i] {
                return i as i32;
            }
            x -= self.w[i];
        }
        unreachable!()
    }
}

struct Rng32 {
    x: u32,
    y: u32,
    z: u32,
    c: u32,
}

impl Rng32 {
    pub fn new() -> Self {
        Rng32 {
            x: 123456789,
            y: 362436000,
            z: 521288629,
            c: 7654321,
        }
    }

    pub fn next(&mut self) -> u32 {
        self.x = self.x.overflowing_mul(69069).0.overflowing_add(12345).0;

        self.y = self.y ^ (self.y << 13);
        self.y = self.y ^ (self.y >> 17);
        self.y = self.y ^ (self.y << 5);

        let t = 698769069u64 * self.z as u64 + self.c as u64;
        self.c = (t >> 32) as u32;
        self.z = t as u32;
        self.x.overflowing_add(self.y).0.overflowing_add(self.z).0
    }
}
