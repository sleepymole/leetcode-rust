#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_unstable_by(|intr1, intr2| intr1[0].cmp(&intr2[0]));
        let mut erased = 0;
        let mut last = intervals[0][1];
        for i in 1..intervals.len() {
            if intervals[i][0] < last {
                last = last.min(intervals[i][1]);
                erased += 1;
            } else {
                last = intervals[i][1];
            }
        }
        erased
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_erase_overlap_intervals() {
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
            1
        );
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2]]),
            2
        );
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]]),
            0
        );
        assert_eq!(
            Solution::erase_overlap_intervals(vec![vec![0, 1], vec![3, 4], vec![1, 2]]),
            0
        );
    }
}
