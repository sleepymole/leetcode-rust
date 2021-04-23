#![allow(dead_code)]
pub struct Solution;

impl Solution {
    fn search<F>(n: usize, f: F) -> usize
    where
        F: Fn(usize) -> bool,
    {
        let (mut i, mut j) = (0, n);
        while i < j {
            let h = (i + j) / 2;
            if f(h) {
                j = h;
            } else {
                i = h + 1;
            }
        }
        i
    }

    fn search_range<F>(l: usize, r: usize, f: F) -> usize
    where
        F: Fn(usize) -> bool,
    {
        Solution::search(r - l, |i| f(i + l)) + l
    }

    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let l = (nums1[0] + nums2[0]) as usize;
        let r = (nums1[nums1.len() - 1] + nums2[nums2.len() - 1]) as usize;
        let max_sum = Solution::search_range(l, r, |target| {
            let mut count = 0;
            for &x in &nums1 {
                count += Solution::search(nums2.len(), |i| x + nums2[i] > target as i32);
            }
            count >= k
        });
        let mut pairs = vec![];
        for &x in &nums1 {
            for j in 0..Solution::search(nums2.len(), |i| x + nums2[i] > max_sum as i32) {
                pairs.push(vec![x, nums2[j]]);
            }
        }
        pairs.sort_unstable_by(|a, b| (a[0] + a[1]).cmp(&(b[0] + b[1])));
        pairs.truncate(k);
        pairs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_smallest_pairs() {
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 7, 11], vec![2, 4, 6], 3),
            vec![vec![1, 2], vec![1, 4], vec![1, 6]]
        );
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 1, 2], vec![1, 2, 3], 2),
            [[1, 1], [1, 1]]
        );
        assert_eq!(
            Solution::k_smallest_pairs(vec![1, 2], vec![3], 3),
            [[1, 3], [2, 3]]
        );
    }
}
