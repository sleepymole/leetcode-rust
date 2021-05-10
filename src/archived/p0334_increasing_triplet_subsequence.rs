#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
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
            if f.len() >= 3 {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increasing_triplet() {
        assert_eq!(Solution::increasing_triplet(vec![1, 2, 3, 4, 5]), true);
        assert_eq!(Solution::increasing_triplet(vec![5, 4, 3, 2, 1]), false);
        assert_eq!(Solution::increasing_triplet(vec![2, 1, 5, 0, 4, 6]), true);
    }
}
