#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let n = intervals.len();
        let mut indices = Vec::new();
        for i in 0..n {
            indices.push(i);
        }
        indices.sort_unstable_by(|&i, &j| intervals[i][0].cmp(&intervals[j][0]));
        let mut res = Vec::new();
        for intr in &intervals {
            let (mut l, mut r) = (0, n);
            while l < r {
                let m = (l + r) / 2;
                let v = intervals[indices[m]][0];
                if v >= intr[1] {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
            res.push(if l < n { indices[l] as i32 } else { -1 });
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_right_interval() {
        assert_eq!(Solution::find_right_interval(vec![vec![1, 2]]), vec![-1]);
        assert_eq!(
            Solution::find_right_interval(vec![vec![3, 4], vec![2, 3], vec![1, 2]]),
            vec![-1, 0, 1]
        );
        assert_eq!(
            Solution::find_right_interval(vec![vec![1, 4], vec![2, 3], vec![3, 4]]),
            vec![-1, 2, -1]
        );
    }
}
