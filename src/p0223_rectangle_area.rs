#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]
pub struct Solution {}

impl Solution {
    pub fn compute_area(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
        let mut s = (c - a).abs() * (d - b).abs() + (g - e).abs() * (h - f).abs();
        let m = c.min(g) as i64 - a.max(e) as i64;
        let n = d.min(h) as i64 - b.max(f) as i64;
        if m >= 0 && n >= 0 {
            s = (s as i64 - m * n) as i32;
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_area() {
        assert_eq!(Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
        assert_eq!(
            Solution::compute_area(-1500000001, 0, -1500000000, 1, 1500000000, 0, 1500000001, 1),
            2
        );
    }
}
