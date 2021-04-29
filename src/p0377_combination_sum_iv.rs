#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        let mut f = Vec::new();
        f.resize(target + 1, 0);
        f[0] = 1;
        for i in 0..target {
            for &x in &nums {
                let x = x as usize;
                if i + x <= target {
                    f[i + x] += f[i];
                }
            }
        }
        f[target]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum4() {
        assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7);
        assert_eq!(Solution::combination_sum4(vec![9], 4), 0);
    }
}
