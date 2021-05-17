#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn is_valid_serialization(preorder: String) -> bool {
        let mut n = 0;
        let mut ok = false;
        for s in preorder.rsplit(',') {
            if s == "#" {
                n += 1;
                if n == 1 {
                    ok = true;
                }
            } else if s.parse::<i32>().ok().is_some() {
                if n >= 2 {
                    n -= 1;
                    ok = true;
                } else {
                    n += 1;
                    ok = false
                }
            } else {
                return false;
            }
        }
        ok && n == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_serialization() {
        assert!(Solution::is_valid_serialization(
            "9,3,4,#,#,1,#,#,2,#,6,#,#".to_owned()
        ));
        assert!(!Solution::is_valid_serialization("1,#".to_owned()));
        assert!(!Solution::is_valid_serialization("9,#,#,1".to_owned()));
    }
}
