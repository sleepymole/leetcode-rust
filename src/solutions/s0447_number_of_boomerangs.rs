#![allow(dead_code)]
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut res = 0;
        for i in 0..n {
            let mut m: HashMap<i32, i32> = HashMap::new();
            for j in 0..n {
                if i == j {
                    continue;
                }
                let x = points[j][0] - points[i][0];
                let y = points[j][1] - points[i][1];
                let d = x * x + y * y;
                *m.entry(d).or_insert(0) += 1;
            }
            for v in m.values() {
                res += v * (v - 1);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_boomerangs() {
        assert_eq!(
            Solution::number_of_boomerangs(vec![vec![0, 0], vec![1, 0], vec![2, 0]]),
            2
        );
        assert_eq!(
            Solution::number_of_boomerangs(vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
            2
        );
        assert_eq!(Solution::number_of_boomerangs(vec![vec![1, 1]]), 0);
    }
}
