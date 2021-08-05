#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n = nums.len();
        let mut res = vec![];
        for i in 0..(1 << n) {
            let mut prev: Option<i32> = None;
            let mut ok = true;
            let mut seq = vec![];
            for j in 0..n {
                if i & (1 << j) > 0 {
                    if let Some(&p) = prev.as_ref() {
                        if nums[j] < p {
                            ok = false;
                            break;
                        }
                    }
                    prev = Some(nums[j]);
                    seq.push(nums[j]);
                }
            }
            if ok && seq.len() >= 2 {
                res.push(seq);
            }
        }
        res.sort_unstable();
        let mut uniq = 0;
        let mut i = 0;
        while i < res.len() {
            let mut j = i + 1;
            while j < res.len() && res[j] == res[j - 1] {
                j += 1;
            }
            res[uniq] = res[i].clone();
            uniq += 1;
            i = j;
        }
        res.truncate(uniq);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_subsequences() {
        assert_eq!(
            Solution::find_subsequences(vec![4, 6, 7, 7]),
            vec![
                vec![4, 6],
                vec![4, 6, 7],
                vec![4, 6, 7, 7],
                vec![4, 7],
                vec![4, 7, 7],
                vec![6, 7],
                vec![6, 7, 7],
                vec![7, 7]
            ]
        );
        assert_eq!(
            Solution::find_subsequences(vec![4, 4, 3, 2, 1]),
            vec![vec![4, 4]]
        );
    }
}
