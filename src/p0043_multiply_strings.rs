#![allow(dead_code)]
pub struct Solution;

use std::char;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let num1: Vec<i32> = num1
            .into_bytes()
            .into_iter()
            .map(|x| x as i32 - '0' as i32)
            .rev()
            .collect();
        let num2: Vec<i32> = num2
            .into_bytes()
            .into_iter()
            .map(|x| x as i32 - '0' as i32)
            .rev()
            .collect();
        let mut num3: Vec<i32> = Vec::new();
        num3.resize(num1.len() + num2.len(), 0);
        for i in 0..num1.len() {
            for j in 0..num2.len() {
                num3[i + j] += num1[i] * num2[j];
            }
        }
        for i in 0..usize::max(num3.len(), 1) - 1 {
            num3[i + 1] += num3[i] / 10;
            num3[i] %= 10;
        }
        while num3.len() > 1 && *num3.last().unwrap() == 0 {
            num3.pop();
        }
        num3.into_iter()
            .map(|x| char::from_digit(x as u32, 10).unwrap())
            .rev()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() {
        assert_eq!(
            Solution::multiply("2".to_owned(), "3".to_owned()),
            "6".to_owned()
        );
        assert_eq!(
            Solution::multiply("123".to_owned(), "456".to_owned()),
            "56088".to_owned(),
        );
        assert_eq!(
            Solution::multiply("0".to_owned(), "0".to_owned()),
            "0".to_owned()
        );
    }
}
