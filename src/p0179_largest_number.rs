#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums;
        nums.sort_unstable_by(|a, b| {
            let mut s1 = a.to_string();
            let mut s2 = b.to_string();
            s1.push_str(s2.as_str());
            s2.push_str(s1.as_str());
            s2.cmp(&s1)
        });
        let ans = nums.iter().fold("".to_owned(), |mut ans, &val| {
            if !ans.is_empty() || val != 0 {
                ans.push_str(val.to_string().as_str());
            }
            ans
        });
        if !ans.is_empty() {
            ans
        } else {
            "0".to_owned()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_number() {
        assert_eq!(Solution::largest_number(vec![10, 2]), "210".to_owned());
        assert_eq!(
            Solution::largest_number(vec![3, 30, 34, 5, 9]),
            "9534330".to_owned()
        );
        assert_eq!(Solution::largest_number(vec![1]), "1".to_owned());
        assert_eq!(Solution::largest_number(vec![10]), "10".to_owned());
    }
}
