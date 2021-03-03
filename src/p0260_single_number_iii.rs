#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let xor = nums.iter().fold(0, |x, v| x ^ v);
        if nums.iter().filter(|&&v| v == 0).count() == 1 {
            return vec![0, xor];
        }
        for i in 0..32 {
            let mut x = 0;
            let mut y = 0;
            for v in &nums {
                if v & (1 << i) != 0 {
                    x ^= v;
                } else {
                    y ^= v;
                }
            }
            if x != 0 && y != 0 {
                return vec![x, y];
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        nums
    }

    #[test]
    fn test_single_number() {
        assert_eq!(
            sorted(Solution::single_number(vec![1, 2, 1, 3, 2, 5])),
            vec![3, 5]
        );
        assert_eq!(sorted(Solution::single_number(vec![-1, 0])), vec![-1, 0]);
        assert_eq!(sorted(Solution::single_number(vec![0, 1])), vec![0, 1]);
    }
}
