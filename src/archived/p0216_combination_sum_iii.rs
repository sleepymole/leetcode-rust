#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        for i in 0..(1 << 9) {
            let mut sum = 0;
            let mut nums = Vec::new();
            for j in 0..9 {
                if i & (1 << j) > 0 {
                    sum += j + 1;
                    nums.push(j + 1);
                }
            }
            if sum == n && nums.len() == k as usize {
                ans.push(nums);
            }
        }
        ans
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
    fn test_combination_sum3() {
        assert_eq!(Solution::combination_sum3(3, 7), vec![vec![1, 2, 4]]);
        assert_eq!(
            normalize(Solution::combination_sum3(3, 9)),
            vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]
        );
        assert_eq!(Solution::combination_sum3(4, 1).is_empty(), true);
        assert_eq!(Solution::combination_sum3(3, 2).is_empty(), true);
        assert_eq!(
            Solution::combination_sum3(9, 45),
            vec![vec![1, 2, 3, 4, 5, 6, 7, 8, 9]]
        );
    }
}
