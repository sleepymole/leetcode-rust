#![allow(dead_code)]
pub struct Solution;

impl Solution {
    fn select(nums: &Vec<i32>, k: usize) -> Vec<i32> {
        let mut res = Vec::new();
        for i in 0..nums.len() {
            while !res.is_empty()
                && res.len() + nums.len() - i > k
                && nums[i] > *res.last().unwrap()
            {
                res.pop();
            }
            res.push(nums[i]);
        }
        res.truncate(k);
        res
    }

    fn merge(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        let (mut i, mut j) = (0, 0);
        while i < nums1.len() || j < nums2.len() {
            if i < nums1.len()
                && (j == nums2.len()
                    || nums1[i] > nums2[j]
                    || nums1[i] == nums2[j] && {
                        let (mut x, mut y) = (i + 1, j + 1);
                        while x < nums1.len() && y < nums2.len() && nums1[x] == nums2[y] {
                            x += 1;
                            y += 1;
                        }
                        y == nums2.len() || x < nums1.len() && nums1[x] > nums2[y]
                    })
            {
                res.push(nums1[i]);
                i += 1;
            } else {
                res.push(nums2[j]);
                j += 1;
            }
        }
        res
    }

    pub fn max_number(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut ans = Vec::new();
        for i in 0..=k {
            if nums1.len() < i || nums2.len() < k - i {
                continue;
            }
            let merged =
                Solution::merge(Solution::select(&nums1, i), Solution::select(&nums2, k - i));
            if merged > ans {
                ans = merged;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_number() {
        assert_eq!(
            Solution::max_number(vec![3, 4, 6, 5], vec![9, 1, 2, 5, 8, 3], 5),
            vec![9, 8, 6, 5, 3]
        );
        assert_eq!(
            Solution::max_number(vec![6, 7], vec![6, 0, 4], 5),
            vec![6, 7, 6, 0, 4]
        );
        assert_eq!(
            Solution::max_number(vec![3, 9], vec![8, 9], 3),
            vec![9, 8, 9]
        );
        assert_eq!(
            Solution::max_number(
                vec![5, 0, 2, 1, 0, 1, 0, 3, 9, 1, 2, 8, 0, 9, 8, 1, 4, 7, 3],
                vec![7, 6, 7, 1, 0, 1, 0, 5, 6, 0, 5, 0],
                31
            ),
            vec![
                7, 6, 7, 5, 1, 0, 2, 1, 0, 1, 0, 5, 6, 0, 5, 0, 1, 0, 3, 9, 1, 2, 8, 0, 9, 8, 1, 4,
                7, 3, 0
            ]
        );
    }
}
