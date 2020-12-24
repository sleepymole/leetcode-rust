#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        for i in (n..m + n).rev() {
            nums1[i as usize] = nums1[(i - n) as usize];
        }
        let (mut i, mut j, mut k) = (n, 0, 0);
        while i < m + n && j < n {
            if nums1[i as usize] <= nums2[j as usize] {
                nums1[k as usize] = nums1[i as usize];
                i += 1;
            } else {
                nums1[k as usize] = nums2[j as usize];
                j += 1;
            }
            k += 1;
        }
        while i < m + n {
            nums1[k as usize] = nums1[i as usize];
            i += 1;
            k += 1;
        }
        while j < n {
            nums1[k as usize] = nums2[j as usize];
            j += 1;
            k += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
        let mut nums1 = vec![1, 2, 4, 5, 6, 0];
        let mut nums2 = vec![3];
        Solution::merge(&mut nums1, 5, &mut nums2, 1);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6]);
    }
}
