#![allow(dead_code)]
pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut m: HashMap<i32, Vec<(usize, usize)>> = HashMap::new();
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                m.entry(target - nums[i] - nums[j])
                    .or_insert_with(Vec::new)
                    .push((i, j));
            }
        }
        let mut s: HashSet<Vec<i32>> = HashSet::new();
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if let Some(v) = m.get(&(nums[i] + nums[j])) {
                    for p in v {
                        if i != p.0 && i != p.1 && j != p.0 && j != p.1 {
                            let mut res = vec![nums[i], nums[j], nums[p.0], nums[p.1]];
                            res.sort_unstable();
                            s.insert(res);
                        }
                    }
                }
            }
        }
        s.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalize(mut quads: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for q in quads.iter_mut() {
            q.sort_unstable();
        }
        quads.sort();
        quads
    }

    #[test]
    fn test_four_sum() {
        let ans = Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0);
        assert_eq!(
            normalize(ans),
            normalize(vec![
                vec![-2, -1, 1, 2],
                vec![-2, 0, 0, 2],
                vec![-1, 0, 0, 1]
            ])
        );
        let ans = Solution::four_sum(vec![-4, -3, -2, -1, 0, 0, 1, 2, 3, 4], 0);
        assert_eq!(
            normalize(ans),
            normalize(vec![
                vec![-4, -3, 3, 4],
                vec![-4, -2, 2, 4],
                vec![-4, -1, 1, 4],
                vec![-4, -1, 2, 3],
                vec![-4, 0, 0, 4],
                vec![-4, 0, 1, 3],
                vec![-3, -2, 1, 4],
                vec![-3, -2, 2, 3],
                vec![-3, -1, 0, 4],
                vec![-3, -1, 1, 3],
                vec![-3, 0, 0, 3],
                vec![-3, 0, 1, 2],
                vec![-2, -1, 0, 3],
                vec![-2, -1, 1, 2],
                vec![-2, 0, 0, 2],
                vec![-1, 0, 0, 1]
            ])
        );
    }
}
