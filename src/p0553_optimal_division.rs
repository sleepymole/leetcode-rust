#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {
        if nums.len() == 1 {
            return nums[0].to_string();
        }
        if nums.len() == 2 {
            return format!("{}/{}", nums[0], nums[1]);
        }
        let mut s = format!("{}/({}", nums[0], nums[1]);
        for i in 2..nums.len() {
            s.push('/');
            s.push_str(&nums[i].to_string());
        }
        s.push(')');
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimal_division() {
        assert_eq!(
            Solution::optimal_division(vec![1000, 100, 10, 2]),
            "1000/(100/10/2)".to_owned()
        );
        assert_eq!(
            Solution::optimal_division(vec![2, 3, 4]),
            "2/(3/4)".to_owned()
        );
        assert_eq!(Solution::optimal_division(vec![2]), "2".to_owned());
    }
}
