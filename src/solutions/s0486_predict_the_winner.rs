#![allow(dead_code)]
pub struct Solution;

impl Solution {
    fn first_win(nums: &[i32], first: i32, second: i32, turn: i32) -> bool {
        if nums.is_empty() {
            return first >= second;
        }
        if turn == 0 {
            Solution::first_win(&nums[1..], first + nums[0], second, 1)
                || Solution::first_win(
                    &nums[0..nums.len() - 1],
                    first + nums[nums.len() - 1],
                    second,
                    1,
                )
        } else {
            Solution::first_win(&nums[1..], first, second + nums[0], 0)
                && Solution::first_win(
                    &nums[0..nums.len() - 1],
                    first,
                    second + nums[nums.len() - 1],
                    0,
                )
        }
    }

    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        Solution::first_win(&nums, 0, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict_the_winner() {
        assert!(!Solution::predict_the_winner(vec![1, 5, 2]));
        assert!(Solution::predict_the_winner(vec![1, 5, 233, 7]));
    }
}
