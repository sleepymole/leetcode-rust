#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut top = Vec::new();
        for x in nums {
            if top.contains(&x) {
                continue;
            }
            if top.len() < 3 {
                top.push(x);
            } else if x > top[0] {
                top[0] = x;
            }
            top.sort_unstable();
        }
        if top.len() == 3 {
            top[0]
        } else {
            *top.last().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_third_max() {
        assert_eq!(Solution::third_max(vec![3, 2, 1]), 1);
        assert_eq!(Solution::third_max(vec![1, 2]), 2);
        assert_eq!(Solution::third_max(vec![2, 2, 3, 1]), 1);
    }
}
