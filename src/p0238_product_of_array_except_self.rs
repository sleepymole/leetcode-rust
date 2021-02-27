#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let p1: i32 = nums.iter().product();
        let zero = nums.iter().filter(|&&x| x == 0).count();
        let p2: i32 = if zero == 1 {
            nums.iter().filter(|&&x| x != 0).product()
        } else {
            0
        };
        nums.into_iter()
            .map(|x| if x != 0 { p1 / x } else { p2 })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_product_except_self() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
    }
}
