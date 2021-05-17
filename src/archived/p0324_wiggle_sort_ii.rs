#![allow(dead_code)]
pub struct Solution;

impl Solution {
    fn find_kth(nums: &mut Vec<i32>, k: usize) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        let mut k = k;
        while l < r {
            nums.swap((l + r) / 2, r);
            let mut lt = l;
            for i in l..r {
                if nums[i] < nums[r] {
                    nums.swap(i, lt);
                    lt += 1;
                }
            }
            if lt == l {
                if k == 1 {
                    return nums[r];
                }
                k -= 1;
                r -= 1;
            } else if lt - l >= k {
                r = lt - 1;
            } else {
                k -= lt - l;
                l = lt;
            }
        }
        nums[l]
    }

    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        if nums.len() <= 1 {
            return;
        }
        let n = nums.len() | 1;
        let wi = |i: usize| (2 * i + 1) % n;
        let m = Solution::find_kth(nums, (nums.len() + 1) / 2);
        let (mut l, mut r) = (0, nums.len() - 1);
        let mut i = 0;
        while i <= r {
            if nums[wi(i)] > m {
                nums.swap(wi(i), wi(l));
                l += 1;
                i += 1;
            } else if nums[wi(i)] < m {
                nums.swap(wi(i), wi(r));
                r -= 1;
            } else {
                i += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wiggle_sorted(nums: &Vec<i32>) -> bool {
        for i in 1..nums.len() {
            if i % 2 == 1 && nums[i] <= nums[i - 1] || i % 2 == 0 && nums[i] >= nums[i - 1] {
                return false;
            }
        }
        true
    }

    #[test]
    fn test_wiggle_sort() {
        let mut nums = vec![1, 5, 1, 1, 6, 4];
        Solution::wiggle_sort(&mut nums);
        assert!(wiggle_sorted(&nums));
        let mut nums = vec![1, 3, 2, 2, 3, 1];
        Solution::wiggle_sort(&mut nums);
        assert!(wiggle_sorted(&nums));
        let mut nums = vec![5, 6, 1];
        Solution::wiggle_sort(&mut nums);
        assert!(wiggle_sorted(&nums));
    }
}
