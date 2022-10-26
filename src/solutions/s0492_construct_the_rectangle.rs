#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut l = area;
        let mut r = 1;
        let mut i = 2;
        while i * i <= area {
            if area % i == 0 {
                l = area / i;
                r = i;
            }
            i += 1;
        }
        vec![l, r]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_rectangle() {
        assert_eq!(Solution::construct_rectangle(4), vec![2, 2]);
        assert_eq!(Solution::construct_rectangle(37), vec![37, 1]);
        assert_eq!(Solution::construct_rectangle(122122), vec![427, 286]);
    }
}
