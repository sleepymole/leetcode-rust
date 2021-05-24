#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let (mut a, mut b, mut m, mut n) = (0, 0, 0, 0);
        for &x in &nums {
            if m > 0 && x == a {
                m += 1;
            } else if n > 0 && x == b {
                n += 1;
            } else if m == 0 {
                a = x;
                m = 1;
            } else if n == 0 {
                b = x;
                n = 1;
            } else {
                m -= 1;
                n -= 1;
            }
        }
        let mut ans = Vec::new();
        if m > 0 && nums.iter().filter(|&&x| x == a).count() * 3 > nums.len() {
            ans.push(a);
        }
        if n > 0 && nums.iter().filter(|&&x| x == b).count() * 3 > nums.len() {
            ans.push(b);
        }
        ans.sort_unstable();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_majority_element() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), vec![3]);
        assert_eq!(Solution::majority_element(vec![1]), vec![1]);
        assert_eq!(Solution::majority_element(vec![1, 2]), vec![1, 2]);
        assert!(Solution::majority_element(vec![1, 2, 3, 4]).is_empty());
    }
}
