#![allow(dead_code)]
pub struct Solution;

impl Solution {
    fn unique(results: &mut Vec<Vec<i32>>) {
        let mut n = 1;
        for i in 1..results.len() {
            if results[i] != results[i - 1] {
                results[n] = results[i].clone();
                n += 1;
            }
        }
        results.truncate(n);
    }

    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut f: Vec<Vec<Vec<i32>>> = Vec::new();
        f.resize((target + 1) as usize, vec![]);
        f[0] = vec![vec![]];
        for x in candidates {
            for i in (x..=target).rev() {
                let mut results = f[(i - x) as usize].clone();
                for r in results.iter_mut() {
                    r.push(x);
                    r.sort_unstable();
                }
                f[i as usize].append(&mut results);
                f[i as usize].sort();
                Solution::unique(&mut f[i as usize]);
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
    fn test_combination_sum2() {
        assert_eq!(
            normalize(Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8)),
            normalize(vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]])
        );
        assert_eq!(
            normalize(Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5)),
            normalize(vec![vec![1, 2, 2], vec![5]])
        );
        assert_eq!(
            Solution::combination_sum2(
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                27
            )
            .is_empty(),
            true
        )
    }
}
