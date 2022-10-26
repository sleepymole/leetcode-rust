#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut res = Vec::new();
        let mut num = 1;
        for _ in 0..n {
            res.push(num);
            if num * 10 <= n {
                num *= 10;
            } else {
                if num + 1 > n {
                    num /= 10;
                }
                num += 1;
                while num % 10 == 0 {
                    num /= 10;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexical_order() {
        assert_eq!(
            Solution::lexical_order(13),
            vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
        );
        assert_eq!(Solution::lexical_order(2), vec![1, 2]);
    }
}
