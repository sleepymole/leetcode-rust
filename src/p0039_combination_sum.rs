#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut f: Vec<Vec<Vec<i32>>> = Vec::new();
        f.resize((target + 1) as usize, vec![]);
        f[0] = vec![vec![]];
        for x in candidates {
            for i in (0..=target).rev() {
                for j in 1..=i / x {
                    let mut results = f[(i - j * x) as usize].clone();
                    for r in results.iter_mut() {
                        r.resize(r.len() + j as usize, x);
                    }
                    f[i as usize].append(&mut results);
                }
            }
        }
        f[target as usize].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalize(mut results: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for res in results.iter_mut() {
            res.sort_unstable();
        }
        results.sort();
        results
    }

    #[test]
    fn test_combination_sum() {
        assert_eq!(
            normalize(Solution::combination_sum(vec![2, 3, 6, 7], 7)),
            normalize(vec![vec![2, 2, 3], vec![7]])
        );
        assert_eq!(
            normalize(Solution::combination_sum(vec![2, 3, 5], 8)),
            normalize(vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]])
        );
        assert_eq!(Solution::combination_sum(vec![2], 1).is_empty(), true);
        assert_eq!(
            normalize(Solution::combination_sum(vec![1], 1)),
            vec![vec![1]]
        );
        assert_eq!(
            normalize(Solution::combination_sum(vec![1], 2)),
            vec![vec![1, 1]]
        );
    }
}
