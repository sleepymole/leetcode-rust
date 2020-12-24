#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut i = 0;
        let mut joined = false;
        while i < intervals.len() {
            if joined || intervals[i][1] < new_interval[0] || intervals[i][0] > new_interval[1] {
                ans.push(intervals[i].clone());
                i += 1;
                continue;
            }
            let (mut l, mut r) = (new_interval[0], new_interval[1]);
            let mut j = i;
            while j < intervals.len()
                && intervals[j][1] >= new_interval[0]
                && intervals[j][0] <= new_interval[1]
            {
                l = l.min(intervals[j][0]);
                r = r.max(intervals[j][1]);
                j += 1;
            }
            ans.push(vec![l, r]);
            i = j;
            joined = true;
        }
        if !joined {
            ans.push(new_interval);
            ans.sort();
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        assert_eq!(
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]],
        );
        assert_eq!(
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
        assert_eq!(Solution::insert(vec![], vec![5, 7]), vec![vec![5, 7]],);
        assert_eq!(
            Solution::insert(vec![vec![1, 5]], vec![2, 3]),
            vec![vec![1, 5]],
        );
        assert_eq!(
            Solution::insert(vec![vec![1, 5]], vec![2, 7]),
            vec![vec![1, 7]],
        );
    }
}
