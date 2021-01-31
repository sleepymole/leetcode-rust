#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }
        let start = *nums.iter().min().unwrap();
        let end = *nums.iter().max().unwrap();
        let avg_gap = (end - start - 1) / (nums.len() as i32 - 1) + 1;
        let mut buckets = vec![vec![]; ((end - start) / avg_gap + 1) as usize];
        for x in nums.into_iter() {
            let idx = ((x - start) / avg_gap) as usize;
            let b = &mut buckets[idx];
            b.push(x);
            b.sort_unstable();
            if b.len() > 2 {
                b.remove(1);
            }
        }
        let mut ans = 0;
        let mut last = *buckets[0].last().unwrap();
        for i in 1..buckets.len() {
            if !buckets[i].is_empty() {
                ans = ans.max(*buckets[i].first().unwrap() - last);
                last = *buckets[i].last().unwrap();
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_gap() {
        assert_eq!(Solution::maximum_gap(vec![3, 6, 9, 1]), 3);
        assert_eq!(Solution::maximum_gap(vec![10]), 0);
    }
}
