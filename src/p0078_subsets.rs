#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        for i in 0..(1 << nums.len()) {
            let mut res = Vec::new();
            for j in 0..nums.len() {
                if i & (1 << j) != 0 {
                    res.push(nums[j]);
                }
            }
            ans.push(res);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut vs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for v in vs.iter_mut() {
            v.sort_unstable();
        }
        vs.sort();
        vs
    }

    #[test]
    fn test_subsets() {
        assert_eq!(
            sorted(Solution::subsets(vec![1, 2, 3])),
            sorted(vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3]
            ])
        );
        assert_eq!(
            sorted(Solution::subsets(vec![0])),
            sorted(vec![vec![], vec![0]])
        );
    }
}
