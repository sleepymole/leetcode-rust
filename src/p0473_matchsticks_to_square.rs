#![allow(dead_code)]
pub struct Solution;

use std::collections::HashSet;

impl Solution {
    fn search(
        edges: [i32; 4],
        target: i32,
        used: usize,
        nums: &Vec<i32>,
        cache: &mut HashSet<(i32, i32, i32, i32, usize)>,
    ) -> bool {
        if edges[0] == target {
            return true;
        }
        if cache.contains(&(edges[0], edges[1], edges[2], edges[3], used)) {
            return false;
        }
        for i in 0..nums.len() {
            if used & (1 << i) > 0 {
                continue;
            }
            for j in (0..4).rev() {
                if edges[j] < target {
                    if edges[j] + nums[i] <= target {
                        let mut edges2 = edges;
                        edges2[j] += nums[i];
                        edges2.sort_unstable();
                        if Solution::search(edges2, target, used | (1 << i), nums, cache) {
                            return true;
                        }
                    }
                    break;
                }
            }
        }
        cache.insert((edges[0], edges[1], edges[2], edges[3], used));
        false
    }

    pub fn makesquare(nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % 4 != 0 {
            return false;
        }
        let mut cache = HashSet::new();
        Solution::search([0; 4], sum / 4, 0, &nums, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_makesquare() {
        assert!(Solution::makesquare(vec![1, 1, 2, 2, 2]));
        assert!(!Solution::makesquare(vec![3, 3, 3, 3, 4]));
        assert!(Solution::makesquare(vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
        ]));
        assert!(Solution::makesquare(vec![
            3, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2
        ]));
        assert!(!Solution::makesquare(vec![
            5969561, 8742425, 2513572, 3352059, 9084275, 2194427, 1017540, 2324577, 6810719,
            8936380, 7868365, 2755770, 9954463, 9912280, 4713511
        ]));
    }
}
