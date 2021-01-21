#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut l, mut r) = (0, numbers.len() - 1);
        while l < r {
            if numbers[l] + numbers[r] < target {
                l += 1;
            } else if numbers[l] + numbers[r] > target {
                r -= 1;
            } else {
                return vec![(l + 1) as i32, (r + 1) as i32];
            }
        }
        vec![0, 0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
        assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
    }
}
