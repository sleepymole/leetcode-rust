#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let (mut ans, mut n) = (0, 0);
        for x in nums {
            if n == 0 {
                ans = x;
                n = 1;
            } else if x == ans {
                n += 1;
            } else {
                n -= 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_majority_element() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }
}
