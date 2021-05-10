#![allow(dead_code)]
pub struct Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn can_measure_water(a: i32, b: i32, c: i32) -> bool {
        let mut s = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back((0, 0));
        s.insert((0, 0));
        while let Some((x, y)) = q.pop_front() {
            if x == c || y == c || x + y == c {
                return true;
            }
            if x > 0 && !s.contains(&(0, y)) {
                s.insert((0, y));
                q.push_back((0, y));
            } else if x < a && !s.contains(&(a, y)) {
                s.insert((a, y));
                q.push_back((a, y));
            } else if y > 0 && !s.contains(&(x, 0)) {
                s.insert((x, 0));
                q.push_back((x, 0));
            } else if y < b && !s.contains(&(x, b)) {
                s.insert((x, b));
                q.push_back((x, b));
            } else if x + y <= a && !s.contains(&(x + y, 0)) {
                s.insert((x + y, 0));
                q.push_back((x + y, 0));
            } else if x + y > a && !s.contains(&(a, x + y - a)) {
                s.insert((a, x + y - a));
                q.push_back((a, x + y - a));
            } else if x + y <= b && !s.contains(&(0, x + y)) {
                s.insert((0, x + y));
                q.push_back((0, x + y));
            } else if x + y > b && !s.contains(&(x + y - b, b)) {
                s.insert((x + y - b, b));
                q.push_back((x + y - b, b));
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_measure_water() {
        assert_eq!(Solution::can_measure_water(3, 5, 4), true);
        assert_eq!(Solution::can_measure_water(2, 6, 5), false);
        assert_eq!(Solution::can_measure_water(1, 2, 3), true);
    }
}
