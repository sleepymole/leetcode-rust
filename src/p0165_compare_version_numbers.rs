#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let chars1: Vec<char> = version1.chars().collect();
        let chars2: Vec<char> = version2.chars().collect();
        let (mut i, mut j) = (0, 0);
        while i < chars1.len() || j < chars2.len() {
            let (mut x, mut y) = (0, 0);
            let mut k = i;
            while k < chars1.len() {
                if chars1[k] == '.' {
                    k += 1;
                    break;
                }
                x = x * 10 + chars1[k] as i32 - '0' as i32;
                k += 1;
            }
            i = k;
            let mut k = j;
            while k < chars2.len() {
                if chars2[k] == '.' {
                    k += 1;
                    break;
                }
                y = y * 10 + chars2[k] as i32 - '0' as i32;
                k += 1;
            }
            j = k;
            if x < y {
                return -1;
            } else if x > y {
                return 1;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_version() {
        assert_eq!(
            Solution::compare_version("1.01".to_owned(), "1.001".to_owned()),
            0
        );
        assert_eq!(
            Solution::compare_version("1.0".to_owned(), "1.0.0".to_owned()),
            0
        );
        assert_eq!(
            Solution::compare_version("0.1".to_owned(), "1.1".to_owned()),
            -1
        );
        assert_eq!(
            Solution::compare_version("1.0.1".to_owned(), "1".to_owned()),
            1
        );
        assert_eq!(
            Solution::compare_version("7.5.2.4".to_owned(), "7.5.3".to_owned()),
            -1
        );
    }
}
