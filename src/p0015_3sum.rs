#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn unique(mut x: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if x.is_empty() {
            return x;
        }
        let mut i = 0;
        for j in 1..x.len() {
            if x[j] != x[i] {
                i += 1;
                x[i] = x[j].clone();
            }
        }
        x.truncate(i + 1);
        x
    }
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        for i in 0..nums.len() {
            let (mut l, mut r) = (i + 1, nums.len() - 1);
            while l < r {
                let s = nums[i] + nums[l] + nums[r];
                if s == 0 {
                    let res = vec![nums[i], nums[l], nums[r]];
                    if ans.is_empty() || ans[ans.len() - 1] != res {
                        ans.push(res);
                    }
                    l += 1;
                r -= 1;
                } else if s > 0 {
                    r -= 1;
                } else {
                    l += 1;
                }
            }
        }
        ans.sort();
        Solution::unique(ans)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn brute_force(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = Vec::new();
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                for k in j + 1..nums.len() {
                    if nums[i] + nums[j] + nums[k] == 0 {
                        let mut res = vec![nums[i], nums[j], nums[k]];
                        res.sort();
                        ans.push(res);
                    }
                }
            }
        }
        ans.sort();
        Solution::unique(ans)
    }

    #[test]
    fn test_three_sum() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            brute_force(vec![-1, 0, 1, 2, -1, -4])
        );
        assert_eq!(Solution::three_sum(vec![]).is_empty(), true);
        assert_eq!(Solution::three_sum(vec![0]).is_empty(), true);
        assert_eq!(
            Solution::three_sum(vec![3, 0, -2, -1, 1, 2]),
            brute_force(vec![3, 0, -2, -1, 1, 2])
        );
        assert_eq!(
            Solution::three_sum(vec![-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6]),
            brute_force(vec![-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6])
        );
    }
}
