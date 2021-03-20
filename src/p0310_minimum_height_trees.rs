#![allow(dead_code)]
pub struct Solution;

use std::usize;

impl Solution {
    fn compute_depth(parent: usize, root: usize, graph: &Vec<Vec<usize>>, depth: &mut Vec<usize>) {
        depth[root] = 0;
        for &to in &graph[root] {
            if to == parent {
                continue;
            }
            Solution::compute_depth(root, to, graph, depth);
            depth[root] = depth[root].max(depth[to] + 1);
        }
    }

    fn compute_height(
        parent: usize,
        root: usize,
        herit: usize,
        graph: &Vec<Vec<usize>>,
        depth: &Vec<usize>,
        height: &mut Vec<usize>,
    ) {
        height[root] = herit.max(depth[root]);
        let mut ds = vec![herit];
        for &to in &graph[root] {
            if to == parent {
                continue;
            }
            ds.push(depth[to] + 1);
        }
        let (mut d0, mut d1) = (0, 0);
        for &d in &ds {
            if d > d0 {
                d1 = d0;
                d0 = d;
            } else if d > d1 {
                d1 = d;
            }
        }
        for &to in &graph[root] {
            if to == parent {
                continue;
            }
            let mut herit = d0;
            if depth[to] + 1 == d0 {
                herit = d1;
            }
            Solution::compute_height(root, to, herit + 1, graph, depth, height);
        }
    }

    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for e in edges {
            graph[e[0] as usize].push(e[1] as usize);
            graph[e[1] as usize].push(e[0] as usize);
        }
        let mut depth = vec![0; n];
        let mut height = vec![0; n];
        Solution::compute_depth(usize::MAX, 0, &graph, &mut depth);
        Solution::compute_height(usize::MAX, 0, 0, &graph, &depth, &mut height);
        let min = *height.iter().min().unwrap();
        let mut ans = vec![];
        for i in 0..n {
            if height[i] == min {
                ans.push(i as i32);
            }
        }
        ans.sort_unstable();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min_height_trees() {
        assert_eq!(
            Solution::find_min_height_trees(4, vec![vec![1, 0], vec![1, 2], vec![1, 3]]),
            vec![1]
        );
        assert_eq!(
            Solution::find_min_height_trees(
                6,
                vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]]
            ),
            vec![3, 4]
        );
        assert_eq!(Solution::find_min_height_trees(1, vec![]), vec![0]);
        assert_eq!(
            Solution::find_min_height_trees(2, vec![vec![0, 1]]),
            vec![0, 1]
        );
        assert_eq!(
            Solution::find_min_height_trees(3, vec![vec![0, 1], vec![0, 2]]),
            vec![0]
        );
        assert_eq!(
            Solution::find_min_height_trees(
                8,
                vec![
                    vec![0, 1],
                    vec![1, 2],
                    vec![2, 3],
                    vec![0, 4],
                    vec![4, 5],
                    vec![4, 6],
                    vec![6, 7]
                ]
            ),
            vec![0]
        );
    }
}
