#![allow(dead_code)]
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Solution::gcd(b, a % b)
        }
    }

    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() <= 2 {
            return points.len() as i32;
        }
        let mut ans = 2;
        for i in 0..points.len() {
            let mut m = HashMap::new();
            let mut dup = 0;
            for j in i + 1..points.len() {
                let mut a = points[j][0] - points[i][0];
                let mut b = points[j][1] - points[i][1];
                if a == 0 && b == 0 {
                    dup += 1;
                    continue;
                }
                let gcd = Solution::gcd(a, b);
                a /= gcd;
                b /= gcd;
                *m.entry((a, b)).or_insert(0) += 1;
            }
            if dup + 1 > ans {
                ans = dup + 1;
            }
            for &v in m.values() {
                if v + dup + 1 > ans {
                    ans = v + dup + 1;
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_points() {
        assert_eq!(
            Solution::max_points(vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
            3
        );
        assert_eq!(
            Solution::max_points(vec![
                vec![1, 1],
                vec![3, 2],
                vec![5, 3],
                vec![4, 1],
                vec![2, 3],
                vec![1, 4],
            ]),
            4
        );
    }
}
