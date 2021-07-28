#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let mut res = 0;
        let mut end = -1;
        for t in time_series {
            res += (t + duration - t.max(end + 1)).max(0);
            end = end.max(t + duration - 1)
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_poisoned_duration() {
        assert_eq!(Solution::find_poisoned_duration(vec![1, 4], 2), 4);
        assert_eq!(Solution::find_poisoned_duration(vec![1, 2], 2), 3);
    }
}
