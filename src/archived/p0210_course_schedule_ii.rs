#![allow(dead_code)]
pub struct Solution {}

use std::collections::VecDeque;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as usize;
        let mut edges = vec![vec![]; num_courses];
        let mut in_degree = vec![0; num_courses];
        for p in prerequisites {
            edges[p[1] as usize].push(p[0] as usize);
            in_degree[p[0] as usize] += 1;
        }
        let mut q = VecDeque::new();
        for i in 0..num_courses {
            if in_degree[i] == 0 {
                q.push_back(i);
            }
        }
        let mut order = vec![];
        while let Some(u) = q.pop_front() {
            for &v in &edges[u] {
                in_degree[v] -= 1;
                if in_degree[v] == 0 {
                    q.push_back(v);
                }
            }
            order.push(u as i32);
        }
        if order.len() < num_courses {
            return vec![];
        }
        order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_order(order: Vec<i32>, preqs: Vec<Vec<i32>>) {
        let mut rank = vec![0; order.len()];
        for i in 0..order.len() {
            rank[order[i] as usize] = i;
        }
        for p in preqs {
            assert_eq!(rank[p[0] as usize] > rank[p[1] as usize], true);
        }
    }

    #[test]
    fn test_find_order() {
        let preqs = vec![vec![1, 0]];
        let order = Solution::find_order(2, preqs.clone());
        check_order(order, preqs);
        let preqs = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
        let order = Solution::find_order(4, preqs.clone());
        check_order(order, preqs);
        let preqs = vec![];
        let order = Solution::find_order(1, preqs.clone());
        check_order(order, preqs);
    }
}
