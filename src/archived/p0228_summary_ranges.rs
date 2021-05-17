#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut ans = Vec::new();
        let mut i = 0;
        while i < nums.len() {
            let mut j = i + 1;
            while j < nums.len() && nums[j - 1] + 1 == nums[j] {
                j += 1;
            }
            if j - i == 1 {
                ans.push(nums[i].to_string());
            } else {
                ans.push(format!("{}->{}", nums[i], nums[j - 1]));
            }
            i = j;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summary_ranges() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            vec!["0->2".to_owned(), "4->5".to_owned(), "7".to_owned()]
        );
        assert_eq!(
            Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
            vec!["0", "2->4", "6", "8->9"]
        );
        assert!(Solution::summary_ranges(vec![]).is_empty());
        assert_eq!(Solution::summary_ranges(vec![-1]), vec!["-1".to_owned()]);
        assert_eq!(Solution::summary_ranges(vec![0]), vec!["0".to_owned()]);
    }
}
