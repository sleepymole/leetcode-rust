#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut f: [[bool; 40001]; 2] = [[false; 40001], [false; 40001]];
        f[0][20000] = true;
        let n = nums.len();
        for (i, x) in nums.into_iter().enumerate() {
            let cur = i % 2;
            let next = (i + 1) % 2;
            for j in 0..=40000 {
                f[next][j] = false;
            }
            for j in 0..=40000 {
                if f[cur][j] {
                    if j + x as usize <= 40000 {
                        f[next][j + x as usize] = true;
                    }
                    if j >= x as usize {
                        f[next][j - x as usize] = true;
                    }
                }
            }
        }
        f[n % 2][20000]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_partition() {
        assert!(Solution::can_partition(vec![1, 5, 11, 5]));
        assert!(!Solution::can_partition(vec![1, 2, 3, 5]));
        assert!(Solution::can_partition(vec![99, 2, 3, 98]));
    }
}
