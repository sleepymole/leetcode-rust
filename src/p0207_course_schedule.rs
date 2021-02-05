#![allow(dead_code)]
pub struct Solution {}

use std::collections::VecDeque;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
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
        let mut count = 0;
        while let Some(u) = q.pop_front() {
            for &v in &edges[u] {
                in_degree[v] -= 1;
                if in_degree[v] == 0 {
                    q.push_back(v);
                }
            }
            count += 1;
        }
        count == num_courses
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_finish() {
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0]]), true);
        assert_eq!(Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
        assert_eq!(
            Solution::can_finish(3, vec![vec![1, 0], vec![1, 2], vec![0, 1]]),
            false
        );
    }
}
