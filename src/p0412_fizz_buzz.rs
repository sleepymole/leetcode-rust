#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut ans = Vec::new();
        for i in 1..=n {
            if i % 3 == 0 && i % 5 == 0 {
                ans.push("FizzBuzz".to_owned());
            } else if i % 3 == 0 {
                ans.push("Fizz".to_owned());
            } else if i % 5 == 0 {
                ans.push("Buzz".to_owned());
            } else {
                ans.push(i.to_string());
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizz_buzz() {
        assert_eq!(
            Solution::fizz_buzz(3),
            vec!["1".to_owned(), "2".to_owned(), "Fizz".to_owned()]
        );
        assert_eq!(
            Solution::fizz_buzz(5),
            vec![
                "1".to_owned(),
                "2".to_owned(),
                "Fizz".to_owned(),
                "4".to_owned(),
                "Buzz".to_owned()
            ]
        );
        assert_eq!(
            Solution::fizz_buzz(15),
            vec![
                "1".to_owned(),
                "2".to_owned(),
                "Fizz".to_owned(),
                "4".to_owned(),
                "Buzz".to_owned(),
                "Fizz".to_owned(),
                "7".to_owned(),
                "8".to_owned(),
                "Fizz".to_owned(),
                "Buzz".to_owned(),
                "11".to_owned(),
                "Fizz".to_owned(),
                "13".to_owned(),
                "14".to_owned(),
                "FizzBuzz".to_owned()
            ]
        );
    }
}
