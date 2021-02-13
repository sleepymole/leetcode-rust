#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut k = k as usize;
        let mut nums = nums;
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            if k == 1 {
                return nums[l..=r].iter().copied().max().unwrap();
            }
            let mut gt = l;
            for i in l..r {
                if nums[i] > nums[r] {
                    nums.swap(i, gt);
                    gt += 1;
                }
            }
            if gt == l {
                r -= 1;
                k -= 1;
            } else if k <= gt - l {
                r = gt - 1;
            } else {
                k -= gt - l;
                l = gt;
            }
        }
        nums[l]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_kth_largest() {
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(
            Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
            4
        );
        assert_eq!(Solution::find_kth_largest(vec![-1, 2, 0], 2), 0);
    }
}
