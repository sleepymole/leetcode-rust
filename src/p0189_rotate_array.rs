#![allow(dead_code)]
pub struct Solution;

impl Solution {
    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            Solution::gcd(b, a % b)
        }
    }

    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        if nums.is_empty() || k == 0 {
            return;
        }
        let k = k as usize % nums.len();
        for i in 0..Solution::gcd(k, nums.len()) {
            let mut pos = i;
            loop {
                let next_pos = (pos + nums.len() - k) % nums.len();
                if next_pos == i {
                    break;
                }
                nums.swap(pos, next_pos);
                pos = next_pos;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut nums, 3);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
        let mut nums = vec![-1, -100, 3, 99];
        Solution::rotate(&mut nums, 2);
        assert_eq!(nums, vec![3, 99, -1, -100]);
    }
}
