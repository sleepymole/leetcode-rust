#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        if n == 0 {
            return true;
        }
        let mut n = n;
        let mut last_planted = usize::MAX;
        for i in 0..flowerbed.len() {
            if flowerbed[i] != 0 {
                last_planted = i;
            } else if (last_planted == usize::MAX || i > last_planted + 1)
                && (i + 1 == flowerbed.len() || flowerbed[i + 1] == 0)
            {
                last_planted = i;
                n -= 1;
                if n == 0 {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_place_flowers() {
        assert!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1));
        assert!(!Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2));
    }
}
