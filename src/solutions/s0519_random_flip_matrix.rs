#![allow(dead_code)]

use std::collections::HashMap;

struct Solution {
    rng32: Rng32,
    m: i32,
    n: i32,
    seats: HashMap<i32, i32>,
    remains: i32,
}

impl Solution {
    fn new(m: i32, n: i32) -> Self {
        Solution {
            rng32: Rng32::new(),
            m,
            n,
            seats: HashMap::new(),
            remains: m * n,
        }
    }

    fn flip(&mut self) -> Vec<i32> {
        let i = (self.rng32.next() % (self.remains as u32)) as i32;
        let s = self.seats.get(&i).cloned().unwrap_or(i);
        self.remains -= 1;
        if i < self.remains {
            self.seats.insert(
                i,
                self.seats
                    .get(&self.remains)
                    .cloned()
                    .unwrap_or(self.remains),
            );
        }
        self.seats.remove(&self.remains);
        vec![s / self.n, s % self.n]
    }

    fn reset(&mut self) {
        self.seats.clear();
        self.remains = self.m * self.n;
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
