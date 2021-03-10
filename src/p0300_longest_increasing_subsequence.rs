#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut f: Vec<i32> = Vec::new();
        for &x in &nums {
            let (mut l, mut r) = (0, f.len());
            while l < r {
                let m = (l + r) / 2;
                if f[m] >= x {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
            if l < f.len() {
                f[l] = f[l].min(x);
            } else {
                f.push(x);
            }
        }
        f.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_lis() {
        assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    }
}
