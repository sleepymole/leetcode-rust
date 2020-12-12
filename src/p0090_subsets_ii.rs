#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut ans = vec![vec![]];
        let mut i = 0;
        while i < nums.len() {
            let mut j = i + 1;
            while j < nums.len() && nums[i] == nums[j] {
                j += 1;
            }
            let n = ans.len();
            for k in 0..n {
                let mut s = ans[k].clone();
                for _ in 0..j - i {
                    s.push(nums[i]);
                    ans.push(s.clone());
                }
            }
            i = j;
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sorted(nums: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        for x in nums.iter_mut() {
            x.sort();
        }
        nums.sort();
        nums
    }

    #[test]
    fn test_subsets_with_dup() {
        assert_eq!(
            sorted(Solution::subsets_with_dup(vec![1, 2, 2])),
            sorted(vec![
                vec![2],
                vec![1],
                vec![1, 2, 2],
                vec![2, 2],
                vec![1, 2],
                vec![]
            ])
        );
    }
}
