#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut f = vec![vec![0; n + 1]; m + 1];
        let mut res = 0;
        for s in strs {
            let mut zeros = 0;
            let mut ones = 0;
            for c in s.chars() {
                if c == '0' {
                    zeros += 1;
                } else {
                    ones += 1;
                }
            }
            if zeros > m || ones > n {
                continue;
            }
            for i in (0..=m - zeros).rev() {
                for j in (0..=n - ones).rev() {
                    res = res.max(f[i][j] + 1);
                    f[i + zeros][j + ones] = f[i + zeros][j + ones].max(f[i][j] + 1);
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
    fn test_find_max_form() {
        assert_eq!(
            Solution::find_max_form(
                vec![
                    "10".to_owned(),
                    "0001".to_owned(),
                    "111001".to_owned(),
                    "1".to_owned(),
                    "0".to_owned()
                ],
                5,
                3
            ),
            4
        );
        assert_eq!(
            Solution::find_max_form(vec!["10".to_owned(), "0".to_owned(), "1".to_owned()], 1, 1),
            2
        );
    }
}
