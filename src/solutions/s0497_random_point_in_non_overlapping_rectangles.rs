#![allow(dead_code)]

struct Rng64 {
    x: u64,
    y: u64,
    z: u64,
    c: u64,
}

impl Rng64 {
    pub fn new() -> Self {
        Rng64 {
            x: 1066149217761810u64,
            y: 362436362436362436u64,
            z: 1234567890987654321u64,
            c: 123456123456123456u64,
        }
    }

    pub fn next(&mut self) -> u64 {
        self.x = self
            .x
            .overflowing_mul(6906969069u64)
            .0
            .overflowing_add(1234567u64)
            .0;

        self.y = self.y ^ (self.y << 13);
        self.y = self.y ^ (self.y >> 17);
        self.y = self.y ^ (self.y << 43);

        let t = (self.z << 58).overflowing_add(self.c).0;
        self.c = self.z >> 6;
        self.z = self.z.overflowing_add(t).0;
        if self.z < t {
            self.c = self.c.overflowing_add(1).0;
        }
        self.x.overflowing_add(self.y).0.overflowing_add(self.z).0
    }
}

pub struct Solution {
    rng64: Rng64,
    rects: Vec<Vec<i32>>,
    points: i64,
}

impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let mut points = 0;
        for rect in &rects {
            points += (rect[2] - rect[0] + 1) as i64 * (rect[3] - rect[1] + 1) as i64;
        }
        Solution {
            rng64: Rng64::new(),
            rects,
            points,
        }
    }

    fn pick(&mut self) -> Vec<i32> {
        let mut x = (self.rng64.next() % self.points as u64) as i64;
        for rect in &self.rects {
            let m = (rect[2] - rect[0] + 1) as i64;
            let n = (rect[3] - rect[1] + 1) as i64;
            if m * n <= x {
                x -= m * n;
                continue;
            }
            return vec![rect[0] + (x % m) as i32, rect[1] + (x / m) as i32];
        }
        unreachable!()
    }
}
