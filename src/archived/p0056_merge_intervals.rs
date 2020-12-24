#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort();
        let mut ans = Vec::new();
        let mut i = 0;
        while i < intervals.len() {
            let mut j = i + 1;
            let (l, mut r) = (intervals[i][0], intervals[i][1]);
            while j < intervals.len() && intervals[j][0] <= r {
                r = r.max(intervals[j][1]);
                j += 1;
            }
            ans.push(vec![l, r]);
            i = j
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            [[1, 6], [8, 10], [15, 18]]
        );
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![4, 5]]),
            vec![vec![1, 5]]
        );
    }
}
