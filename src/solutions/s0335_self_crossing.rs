#![allow(dead_code)]
pub struct Solution;

impl Solution {
    fn is_crossing(p0: (i32, i32), p1: (i32, i32), p2: (i32, i32), p3: (i32, i32)) -> bool {
        if p0.1 == p1.1 {
            (p2.0 - p0.0) * (p2.0 - p1.0) <= 0 && (p2.1 - p0.1) * (p3.1 - p0.1) <= 0
        } else {
            (p2.1 - p0.1) * (p2.1 - p1.1) <= 0 && (p2.0 - p0.0) * (p3.0 - p0.0) <= 0
        }
    }

    pub fn is_self_crossing(distance: Vec<i32>) -> bool {
        let mut ring: [(i32, i32); 6] = [(0, 0); 6];
        let (mut start, mut end) = (0, 2);
        let directions = [(0, 1), (-1, 0), (0, -1), (1, 0)];
        for i in 0..distance.len() {
            let nx = ring[(end - 1) % 6].0 + directions[i % 4].0 * distance[i];
            let ny = ring[(end - 1) % 6].1 + directions[i % 4].1 * distance[i];
            if end - start >= 4
                && Solution::is_crossing(
                    ring[(end - 3) % 6],
                    ring[(end - 4) % 6],
                    ring[(end - 1) % 6],
                    (nx, ny),
                )
                || end - start >= 6
                    && Solution::is_crossing(
                        ring[(end - 5) % 6],
                        ring[(end - 6) % 6],
                        ring[(end - 1) % 6],
                        (nx, ny),
                    )
            {
                return true;
            }
            ring[end % 6] = (nx, ny);
            end += 1;
            if end - start > 6 {
                start += 1;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_self_crossing() {
        assert!(Solution::is_self_crossing(vec![2, 1, 1, 2]));
        assert!(!Solution::is_self_crossing(vec![1, 2, 3, 4]));
        assert!(Solution::is_self_crossing(vec![1, 1, 1, 1]));
        assert!(Solution::is_self_crossing(vec![1, 1, 2, 1, 1]));
    }
}
