#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        for i in 0..(1 << n) {
            let mut res = Vec::new();
            for j in 0..n {
                if i & (1 << j) != 0 {
                    res.push(j + 1);
                }
            }
            if res.len() as i32 == k {
                ans.push(res);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut vs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for v in vs.iter_mut() {
            v.sort();
        }
        vs.sort();
        vs
    }

    #[test]
    fn test_combine() {
        assert_eq!(
            sorted(Solution::combine(4, 2)),
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4]
            ]
        );
        assert_eq!(Solution::combine(1, 1), vec![vec![1]]);
    }
}
