#![allow(dead_code)]
pub struct Solution {}

use std::collections::BTreeMap;

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut angles = Vec::new();
        for b in buildings {
            angles.push((b[0], -b[2]));
            angles.push((b[1], b[2]));
        }
        angles.sort_unstable();
        let mut heights = BTreeMap::new();
        let mut points = Vec::new();
        for angle in angles {
            let h1 = heights
                .iter()
                .next_back()
                .map(|(&v, _)| v)
                .unwrap_or_default();
            if angle.1 > 0 {
                *heights.entry(angle.1).or_default() -= 1;
                if let Some(&v) = heights.get(&angle.1) {
                    if v == 0 {
                        heights.remove(&angle.1);
                    }
                }
            } else {
                *heights.entry(-angle.1).or_insert(0) += 1;
            }
            let h2 = heights
                .iter()
                .next_back()
                .map(|(&v, _)| v)
                .unwrap_or_default();
            if h1 != h2 {
                points.push(vec![angle.0, h2]);
            }
        }
        points
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_skyline() {
        assert_eq!(
            Solution::get_skyline(vec![
                vec![2, 9, 10],
                vec![3, 7, 15],
                vec![5, 12, 12],
                vec![15, 20, 10],
                vec![19, 24, 8]
            ]),
            vec![
                vec![2, 10],
                vec![3, 15],
                vec![7, 12],
                vec![12, 0],
                vec![15, 10],
                vec![20, 8],
                vec![24, 0]
            ]
        );
        assert_eq!(
            Solution::get_skyline(vec![vec![0, 2, 3], vec![2, 5, 3]]),
            vec![vec![0, 3], vec![5, 0]]
        );
    }
}
