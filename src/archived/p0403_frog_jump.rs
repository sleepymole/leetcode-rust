#![allow(dead_code)]
pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let mut all_pos = HashSet::new();
        for &pos in &stones {
            all_pos.insert(pos);
        }
        let mut steps: HashMap<i32, HashSet<i32>> = HashMap::new();
        steps.entry(0).or_insert_with(HashSet::new).insert(1);
        let &last = stones.last().unwrap();
        for pos in stones {
            if !steps.contains_key(&pos) {
                continue;
            }
            for u in steps.get(&pos).unwrap().clone() {
                if !all_pos.contains(&(pos + u)) {
                    continue;
                }
                if pos + u == last {
                    return true;
                }
                let next_units = steps.entry(pos + u).or_insert_with(HashSet::new);
                if u > 0 {
                    next_units.insert(u - 1);
                }
                next_units.insert(u);
                next_units.insert(u + 1);
            }
        }
        steps.contains_key(&last)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_cross() {
        assert!(Solution::can_cross(vec![0, 1, 3, 5, 6, 8, 12, 17]));
        assert!(!Solution::can_cross(vec![0, 1, 2, 3, 4, 8, 9, 11]));
    }
}
