#![allow(dead_code)]
pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    fn dp(
        mut l: i8,
        r: i8,
        mut k: i8,
        boxes: &Vec<i32>,
        cache: &mut HashMap<(i8, i8, i8), i32>,
    ) -> i32 {
        while l < r && boxes[l as usize] == boxes[l as usize + 1] {
            k += 1;
            l += 1;
        }
        if l == r {
            return (k as i32) * (k as i32);
        }
        if let Some(&v) = cache.get(&(l, r, k)) {
            return v;
        }
        let mut res = (k as i32) * (k as i32) + Solution::dp(l + 1, r, 1, boxes, cache);
        for j in l + 1..=r {
            if boxes[l as usize] == boxes[j as usize] && boxes[j as usize - 1] != boxes[j as usize]
            {
                res = res.max(
                    Solution::dp(l + 1, j - 1, 1, boxes, cache)
                        + Solution::dp(j, r, k + 1, boxes, cache),
                );
            }
        }
        cache.insert((l, r, k), res);
        res
    }

    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        let mut cache = HashMap::new();
        Solution::dp(0, boxes.len() as i8 - 1, 1, &boxes, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_boxes() {
        assert_eq!(Solution::remove_boxes(vec![1, 3, 2, 2, 2, 3, 4, 3, 1]), 23);
        assert_eq!(Solution::remove_boxes(vec![1, 1, 1]), 9);
        assert_eq!(
            Solution::remove_boxes(vec![
                1, 2, 2, 1, 1, 1, 2, 1, 1, 2, 1, 2, 1, 1, 2, 2, 1, 1, 2, 2, 1, 1, 1, 2, 2, 2, 2, 1,
                2, 1, 1, 2, 2, 1, 2, 1, 2, 2, 2, 2, 2, 1, 2, 1, 2, 2, 1, 1, 1, 2, 2, 1, 2, 1, 2, 2,
                1, 2, 1, 1, 1, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 1, 1,
                1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 2, 1
            ]),
            2758
        );
    }
}
