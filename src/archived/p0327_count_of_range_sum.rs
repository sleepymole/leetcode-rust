#![allow(dead_code)]
#![allow(clippy::manual_memcpy)]
pub struct Solution;

impl Solution {
    fn merge_sort(sum: &mut [i64], lo: usize, hi: usize, lower: i64, upper: i64) -> i64 {
        if hi - lo <= 1 {
            return 0;
        }
        let mid = lo + (hi - lo + 1) / 2;
        let mut count = Solution::merge_sort(sum, lo, mid, lower, upper)
            + Solution::merge_sort(sum, mid, hi, lower, upper);
        let (mut l, mut r) = (lo, lo);
        for i in mid..hi {
            while r < mid && sum[i] - sum[r] >= lower {
                r += 1;
            }
            while l < mid && sum[i] - sum[l] > upper {
                l += 1;
            }
            count += (r - l) as i64;
        }
        let mut sorted = vec![];
        let (mut i, mut j) = (lo, mid);
        while i < mid && j < hi {
            if sum[i] <= sum[j] {
                sorted.push(sum[i]);
                i += 1;
            } else {
                sorted.push(sum[j]);
                j += 1;
            }
        }
        while i < mid {
            sorted.push(sum[i]);
            i += 1;
        }
        while j < hi {
            sorted.push(sum[j]);
            j += 1;
        }
        for i in lo..hi {
            sum[i] = sorted[i - lo];
        }
        count
    }

    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut sum = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            sum[i + 1] = sum[i] + nums[i] as i64;
        }
        Solution::merge_sort(&mut sum, 0, nums.len() + 1, lower as i64, upper as i64) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_range_sum() {
        assert_eq!(Solution::count_range_sum(vec![-2, 5, -1], -2, 2), 3);
        assert_eq!(Solution::count_range_sum(vec![0], 0, 0), 1);
    }
}
