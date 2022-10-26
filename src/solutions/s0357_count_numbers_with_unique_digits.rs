#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        let mut total = 1;
        for i in 1..=n {
            let mut count = 9;
            for j in 1..i {
                count *= 10 - j;
            }
            total += count;
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_numbers_with_unique_digits() {
        assert_eq!(Solution::count_numbers_with_unique_digits(2), 91);
        assert_eq!(Solution::count_numbers_with_unique_digits(0), 1);
    }
}
